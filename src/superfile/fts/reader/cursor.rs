// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: Copyright The Infino Authors

//! Low-level FTS posting cursors: the parsed per-term header/skip table
//! ([`TermMeta`], [`BlockMeta`]) and the block-at-a-time [`TermCursor`]
//! the scorers, phrase walk, and count kernels drive. Scoped `pub(super)`
//! to the `reader/` module — never referenced outside the FTS layer.

use std::sync::Arc;

use bytes::Bytes;

use super::core::{read_u32_le, read_u64_le};
use crate::superfile::{
    ReadError,
    error::FtsError,
    format::{
        self,
        fts::{POSITION_SUBINDEX_STRIDE, U32_BYTES, U64_BYTES, skip_entry, term_meta},
    },
    fts::{
        block256, bm25,
        builder::{
            SKIP_ENTRY_SIZE, SKIP_ENTRY_SIZE_PRE_V5, TERM_META_POSITIONAL_SIZE, TERM_META_SIZE,
        },
        posting,
    },
};

/// Maximum posting-block length across codecs — cursor buffers are sized to this
/// so one cursor can hold either a 128-doc (`posting`) or 256-doc (`block256`)
/// block.
const BLOCK_LEN_MAX: usize = block256::BLOCK_LEN;

/// Which posting-block codec a superfile uses, selected by its FTS version:
/// `V1`–`V4` → 128-doc blocks (`posting` / `BitPacker4x`); `V5` → 256-doc blocks
/// (`block256`, the in-tree codec). One reader binary handles both — the block size,
/// decode routine, and a couple of header-field offsets differ between the two.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum PostingCodec {
    Block128,
    Block256,
}

impl PostingCodec {
    /// Select the codec from the FTS blob version.
    #[inline]
    pub(super) fn from_version(version: u32) -> Self {
        if version == format::fts::VERSION_V5 {
            PostingCodec::Block256
        } else {
            PostingCodec::Block128
        }
    }

    #[inline]
    pub(super) fn block_len(self) -> usize {
        match self {
            PostingCodec::Block128 => posting::BLOCK_LEN,
            PostingCodec::Block256 => block256::BLOCK_LEN,
        }
    }

    /// Decode a block's doc ids + tfs; `dest`s must be `>= BLOCK_LEN_MAX`.
    #[inline]
    pub(super) fn decode_block(self, bytes: &[u8], doc_ids: &mut [u32], tfs: &mut [u32]) -> usize {
        match self {
            PostingCodec::Block128 => posting::decode_block(bytes, doc_ids, tfs),
            PostingCodec::Block256 => block256::decode_block(bytes, doc_ids, tfs),
        }
    }

    /// Decode only a block's doc ids; `dest` must be `>= BLOCK_LEN_MAX`.
    #[inline]
    pub(super) fn decode_block_doc_ids(self, bytes: &[u8], doc_ids: &mut [u32]) -> usize {
        match self {
            PostingCodec::Block128 => posting::decode_block_doc_ids(bytes, doc_ids),
            PostingCodec::Block256 => block256::decode_block_doc_ids(bytes, doc_ids),
        }
    }

    /// Header byte offset of the block `encoding` discriminant.
    #[inline]
    pub(super) fn encoding_off(self) -> usize {
        match self {
            PostingCodec::Block128 => posting::ENCODING_OFF,
            PostingCodec::Block256 => block256::ENCODING_OFF,
        }
    }

    /// Header byte offset of `tf_bits`.
    #[inline]
    pub(super) fn tf_bits_off(self) -> usize {
        match self {
            PostingCodec::Block128 => posting::TF_BITS_OFF,
            PostingCodec::Block256 => block256::TF_BITS_OFF,
        }
    }

    /// `encoding` byte value for a bitset block (identical in both codecs).
    #[inline]
    pub(super) fn encoding_bitset(self) -> u8 {
        match self {
            PostingCodec::Block128 => posting::ENCODING_BITSET,
            PostingCodec::Block256 => block256::ENCODING_BITSET,
        }
    }

    /// Position sub-index entries per block (`block_len / stride`).
    #[inline]
    pub(super) fn subindex_entries_per_block(self) -> usize {
        self.block_len() / POSITION_SUBINDEX_STRIDE
    }

    /// Skip-table entry size. V5 (256-doc) entries carry two extra fields (the
    /// second 128-half's block-max and the first half's last doc id) for
    /// half-granular ranked pruning; pre-V5 (128-doc) entries do not.
    #[inline]
    pub(super) fn skip_entry_size(self) -> usize {
        match self {
            PostingCodec::Block128 => SKIP_ENTRY_SIZE_PRE_V5,
            PostingCodec::Block256 => SKIP_ENTRY_SIZE,
        }
    }

    /// Whether this codec's skip entries carry the V5 sub-block bound fields.
    #[inline]
    pub(super) fn has_sub_block_bounds(self) -> bool {
        matches!(self, PostingCodec::Block256)
    }
}

/// Parsed per-(column, term) metadata header from the postings
/// region. The byte layout is documented once, on the writer side —
/// see [`TERM_META_SIZE`] in `builder.rs` — this struct is its
/// read-side mirror and must stay in sync with that doc.
///
/// [`TermMeta::parse`] is the single place that validates untrusted
/// offsets (the FST value points here) against the postings region:
/// both the fixed 20-byte header and the skip table it declares are
/// bounds-checked before any caller touches a byte. Both the
/// single-term BMW path and [`TermCursor::new`] go through here, so
/// the header layout is interpreted in exactly one spot.
#[derive(Debug, Copy, Clone)]
pub(super) struct TermMeta {
    /// Document frequency — number of docs containing the term.
    pub(super) df: u64,
    /// Byte length of the term's whole region (header + skip table +
    /// blocks), relative to the term's `metadata_offset`.
    pub(super) postings_length: usize,
    /// Number of PFOR blocks (= number of skip-table entries).
    pub(super) num_blocks: usize,
    /// Absolute offset (within the postings region) of the first
    /// skip-table entry: `metadata_offset + TERM_META_SIZE`.
    pub(super) skip_start: usize,
    /// This term's byte offset in the positions region (positional
    /// columns; zero otherwise).
    pub(super) positions_offset: u64,
    /// Byte length of this term's position runs (positional columns;
    /// zero otherwise).
    pub(super) positions_length: u32,
    /// Absolute offset (within the postings region) of this term's
    /// position run-offset sub-index — the block of
    /// `num_blocks × ENTRIES_PER_BLOCK` `u32`s sitting right after the
    /// skip table on a `VERSION_V3` positional term. `None` on
    /// `V1`/`V2` (no sub-index) and on positionless terms.
    pub(super) subindex_start: Option<usize>,
    /// Position sub-index entries per posting block for this blob's codec
    /// (`block_len / POSITION_SUBINDEX_STRIDE`: 8 for 128-doc blocks, 16 for
    /// 256-doc). Stored so the sub-index layout is read with the right stride
    /// regardless of the blob's block size.
    pub(super) subindex_entries_per_block: usize,
    /// Skip-table entry size for this blob's codec (16 pre-V5, 24 for V5).
    pub(super) skip_entry_size: usize,
    /// Whether skip entries carry the V5 sub-block (128-half) bound fields.
    pub(super) has_sub_block_bounds: bool,
}

impl TermMeta {
    /// Parse + bounds-validate the header and its skip table.
    /// Returns `Err` (never panics) on a corrupt or malicious
    /// `metadata_offset` — the crate-wide "untrusted input yields
    /// `Err`, not a slice-index panic" rule.
    pub(super) fn parse(
        postings: &[u8],
        metadata_offset: usize,
        positional: bool,
        has_subindex: bool,
        entries_per_block: usize,
        skip_entry_size: usize,
        has_sub_block_bounds: bool,
    ) -> Result<Self, FtsError> {
        // Positional columns carry the extended 32-byte header (the
        // term's positions offset + length after `num_blocks`); the
        // skip table starts after whichever stride applies. The
        // positions fields themselves are consumed by the phrase read
        // path, not here.
        let term_meta_size = match positional {
            true => TERM_META_POSITIONAL_SIZE,
            false => TERM_META_SIZE,
        };
        if metadata_offset + term_meta_size > postings.len() {
            return Err(FtsError::Read(ReadError::MalformedVersion(
                "term metadata offset out of postings region".into(),
            )));
        }
        let df = read_u32_le(
            &postings[metadata_offset + term_meta::DF_OFF
                ..metadata_offset + term_meta::DF_OFF + U32_BYTES],
        ) as u64;
        // bytes [4..12] = self-offset (redundant; u64); skip
        let postings_length = read_u32_le(
            &postings[metadata_offset + term_meta::POSTINGS_LENGTH_OFF
                ..metadata_offset + term_meta::POSTINGS_LENGTH_OFF + U32_BYTES],
        ) as usize;
        let num_blocks = read_u32_le(
            &postings[metadata_offset + term_meta::NUM_BLOCKS_OFF
                ..metadata_offset + term_meta::NUM_BLOCKS_OFF + U32_BYTES],
        ) as usize;

        let (positions_offset, positions_length) = match positional {
            true => (
                read_u64_le(
                    &postings[metadata_offset + term_meta::POSITIONS_OFFSET_OFF
                        ..metadata_offset + term_meta::POSITIONS_OFFSET_OFF + U64_BYTES],
                ),
                read_u32_le(
                    &postings[metadata_offset + term_meta::POSITIONS_LENGTH_OFF
                        ..metadata_offset + term_meta::POSITIONS_LENGTH_OFF + U32_BYTES],
                ),
            ),
            false => (0, 0),
        };

        // The last block's end offset comes straight from
        // `postings_length`; bound it now instead of slicing OOB later.
        if metadata_offset + postings_length > postings.len() {
            return Err(FtsError::Read(ReadError::MalformedVersion(
                "term postings length exceeds the fetched term range".into(),
            )));
        }
        let skip_start = metadata_offset + term_meta_size;
        let skip_end = skip_start + num_blocks * skip_entry_size;
        if skip_end > postings.len() {
            return Err(FtsError::Read(ReadError::MalformedVersion(
                "skip table runs past postings region".into(),
            )));
        }
        // v3 positional terms store a run-offset sub-index right after the
        // skip table: `num_blocks × ENTRIES_PER_BLOCK` u32s. Bound it now;
        // the blocks follow it (their offsets are read from the skip
        // table, which the writer already shifted past the sub-index).
        let subindex_start = match has_subindex {
            true => {
                let subindex_end = skip_end + num_blocks * entries_per_block * U32_BYTES;
                if subindex_end > postings.len() {
                    return Err(FtsError::Read(ReadError::MalformedVersion(
                        "position sub-index runs past postings region".into(),
                    )));
                }
                Some(skip_end)
            }
            false => None,
        };
        Ok(Self {
            df,
            postings_length,
            num_blocks,
            skip_start,
            positions_offset,
            positions_length,
            subindex_start,
            subindex_entries_per_block: entries_per_block,
            skip_entry_size,
            has_sub_block_bounds,
        })
    }

    /// For a `VERSION_V3` positional term, the run offset of the nearest
    /// sub-index checkpoint at or before pair `pair_in_block` of block
    /// `block`, and the number of runs to skip from it to reach the pair.
    /// The offset is relative to the term's positions (like
    /// [`Self::positions_block_offset`]). `None` when there is no
    /// sub-index (`V1`/`V2`) — the caller falls back to the block-start
    /// walk. The skip is always `< POSITION_SUBINDEX_STRIDE`.
    #[inline]
    pub(super) fn positions_subindex_offset(
        &self,
        postings: &[u8],
        block: usize,
        pair_in_block: usize,
    ) -> Option<(u32, usize)> {
        let start = self.subindex_start?;
        let slot = pair_in_block / POSITION_SUBINDEX_STRIDE;
        let idx = block * self.subindex_entries_per_block + slot;
        let at = start + idx * U32_BYTES;
        let checkpoint = read_u32_le(&postings[at..at + U32_BYTES]);
        let runs_to_skip = pair_in_block % POSITION_SUBINDEX_STRIDE;
        Some((checkpoint, runs_to_skip))
    }

    /// Decode skip-table entry `i` into `(last_doc_id, block_offset_in_term,
    /// block_max_lo, block_max_hi, mid_last_doc_id)`. `block_offset_in_term` is
    /// relative to the term's `metadata_offset`. `block_max_lo`/`block_max_hi`
    /// are the per-128-half BM25 upper bounds recovered from the fixed-point
    /// fields; on pre-V5 blobs (no sub-block bounds) `block_max_hi == block_max_lo`
    /// and `mid_last_doc_id == last_doc_id` (one whole-block bound, half never
    /// selected). Per-entry on purpose — the single-term BMW walk streams entries
    /// without materializing a `Vec`.
    #[inline]
    pub(super) fn skip_entry(&self, postings: &[u8], i: usize) -> (u32, usize, f32, f32, u32) {
        debug_assert!(i < self.num_blocks, "skip entry {i} >= {}", self.num_blocks);
        let entry_off = self.skip_start + i * self.skip_entry_size;
        let last_doc_id = read_u32_le(
            &postings[entry_off + skip_entry::LAST_DOC_ID_OFF
                ..entry_off + skip_entry::LAST_DOC_ID_OFF + U32_BYTES],
        );
        let block_offset = read_u32_le(
            &postings[entry_off + skip_entry::BLOCK_OFFSET_OFF
                ..entry_off + skip_entry::BLOCK_OFFSET_OFF + U32_BYTES],
        ) as usize;
        // Decode a fixed-point field to a guaranteed upper bound on the block's
        // BM25. The builder ceil()s on encode, but `x1000 as f32 / SCALE` can
        // still round a hair below the true max (f32 division), and superfiles
        // written before the encode-side ceil truncated outright. Add one
        // fixed-point step before unscaling so the decoded bound is always
        // >= the true block max. This matters for the cross-superfile floor:
        // block-skip compares `block_max <= floor`, and a bound that dips below
        // a score-tied block's true max would let a rising floor skip that
        // block, dropping tied hits by completion order (nondeterministic
        // top-k). The +1 step costs ~1/SCALE of pruning tightness — negligible.
        let decode_bound = |off: usize| -> f32 {
            let x1000 = read_u32_le(&postings[entry_off + off..entry_off + off + U32_BYTES]);
            x1000.saturating_add(1) as f32 / format::fts::BLOCK_MAX_BM25_FIXED_POINT_SCALE
        };
        let block_max_lo = decode_bound(skip_entry::MAX_BM25_OFF);
        let (block_max_hi, mid_last_doc_id) = if self.has_sub_block_bounds {
            (
                decode_bound(skip_entry::MAX_BM25_HI_OFF),
                read_u32_le(
                    &postings[entry_off + skip_entry::MID_LAST_DOC_ID_OFF
                        ..entry_off + skip_entry::MID_LAST_DOC_ID_OFF + U32_BYTES],
                ),
            )
        } else {
            (block_max_lo, last_doc_id)
        };
        (
            last_doc_id,
            block_offset,
            block_max_lo,
            block_max_hi,
            mid_last_doc_id,
        )
    }

    /// This block's position-run byte offset within the term's
    /// positions bytes — the skip entry's fourth field (zero on
    /// positionless columns, where it is the reserved slot).
    #[inline]
    pub(super) fn positions_block_offset(&self, postings: &[u8], i: usize) -> u32 {
        debug_assert!(i < self.num_blocks, "skip entry {i} >= {}", self.num_blocks);
        let entry_off = self.skip_start + i * self.skip_entry_size;
        read_u32_le(
            &postings[entry_off + skip_entry::POSITIONS_BLOCK_OFFSET_OFF
                ..entry_off + skip_entry::POSITIONS_BLOCK_OFFSET_OFF + U32_BYTES],
        )
    }

    /// End offset (relative to the term's `metadata_offset`) of block
    /// `i`'s bytes. Blocks are concatenated back-to-back, so each
    /// block ends where the next one's `block_offset` begins; the last
    /// block ends at `postings_length`.
    #[inline]
    pub(super) fn block_end_in_term(&self, postings: &[u8], i: usize) -> usize {
        if i + 1 < self.num_blocks {
            let next_off = self.skip_start + (i + 1) * self.skip_entry_size;
            read_u32_le(&postings[next_off + 4..next_off + 8]) as usize
        } else {
            self.postings_length
        }
    }
}

/// Per-term per-block metadata, parsed once at `TermCursor` construction.
#[derive(Debug, Clone, Copy)]
pub(super) struct BlockMeta {
    /// Largest doc_id present in this block.
    pub(super) last_doc_id: u32,
    /// Absolute byte offset (within the FTS postings region) of this
    /// block's encoded bytes.
    pub(super) block_byte_offset: usize,
    /// Absolute byte offset of the first byte AFTER this block. For
    /// the last block of a term it's `metadata_offset + postings_length`.
    pub(super) block_byte_end: usize,
    /// BM25 upper bound over this block's first 128-doc half (whole-block bound
    /// on pre-V5 blobs). Recovered from the skip table's fixed-point field.
    pub(super) block_max_bm25_lo: f32,
    /// BM25 upper bound over this block's second 128-doc half. Equals
    /// `block_max_bm25_lo` on pre-V5 blobs (no second half).
    pub(super) block_max_bm25_hi: f32,
    /// Last doc-id of the first 128-doc half — the split point for choosing
    /// which half's bound applies. Equals `last_doc_id` on pre-V5 blobs, so the
    /// second half is never selected.
    pub(super) mid_last_doc_id: u32,
}

impl BlockMeta {
    /// Whole-block BM25 upper bound (the max of the two half bounds).
    #[inline(always)]
    pub(super) fn block_max_bm25(&self) -> f32 {
        self.block_max_bm25_lo.max(self.block_max_bm25_hi)
    }
}

/// Per-query-term cursor used by [`FtsReader::run_max_score_bmm`]
/// (and by [`FtsReader::run_wand_bmw`] in the bench-only path).
///
/// State:
///   - `blocks`: parsed skip table — one entry per block, lets us
///     decide whether to decode a block before paying the cost.
///   - `current_block` + `pos`: where we are in the term's posting
///     list. `pos == block_n` is treated as "advance to next block".
///   - `block_doc_ids` / `block_tfs`: decoded buffers for the current
///     block, reused across blocks.
///
/// `current_doc_id() == u32::MAX` is the "exhausted" sentinel; the
/// WAND loop drops cursors that are exhausted at the top of each
/// iteration.
#[derive(Clone)]
pub(crate) struct TermCursor {
    /// Precomputed `idf * (K1 + 1)` — the score numerator's
    /// per-cursor constant. Computed once at cursor build so the
    /// hot inner loop fits one multiply + add + divide per call.
    /// (The bare `idf` value isn't kept on the cursor — every hot
    /// scoring path uses `score_with_dl_norm_k1` which takes
    /// `idf_x_k1p1` directly.)
    pub(super) idf_x_k1p1: f32,
    /// Maximum block-max-BM25 across all blocks. Used by the WAND
    /// pivot test (term-level upper bound).
    pub(super) term_max_bm25: f32,
    /// Document frequency of the term (postings list length). Used by
    /// the 2-term OR router to detect a rare anchor term (short list),
    /// where WAND+BMW can skip the other term's long list.
    pub(super) df: u64,
    /// Per-block metadata (the parsed skip table). Read-only after
    /// build and `Arc`-shared, so cloning a cursor for another doc-id
    /// sub-range costs the ~1 KiB decode buffers, never a re-parse.
    pub(super) blocks: Arc<[BlockMeta]>,
    /// Decoded buffers for the current block. Reused across decodes.
    pub(super) block_doc_ids: Vec<u32>,
    pub(super) block_tfs: Vec<u32>,
    /// Number of valid entries in the decoded block buffers (the
    /// last block may be partial).
    pub(super) block_n: usize,
    /// Index into `blocks` of the currently-decoded block. Equal to
    /// `blocks.len()` once exhausted.
    pub(super) current_block: usize,
    /// Position within the currently-decoded block. Always `<
    /// block_n` while not exhausted.
    pub(super) pos: usize,
    /// Index into `blocks` of the block being inspected by the BMW
    /// upper-bound check. Standard block-cursor split:
    /// `shallow_advance_block_to(pivot_doc)` updates this without
    /// decoding the block, so subsequent BMW UB lookups for
    /// monotonically-increasing pivot docs are amortized O(1). Always
    /// `>= current_block`; synced up whenever `current_block` is
    /// advanced.
    pub(super) inspect_block: usize,
    /// The doc most recently passed to `shallow_advance_block_to` — the target
    /// the inspect-block pointer was advanced to. Chooses which 128-doc half's
    /// bound the `inspect_block_*` methods report, so ranked pruning is
    /// half-granular even though blocks decode 256-wide.
    pub(super) inspect_target: u32,
    /// This term's own postings bytes — the metadata header (offset
    /// 0), skip table, and encoded blocks, fetched as a single
    /// contiguous range by [`FtsReader::fetch_term_postings`]. All
    /// `BlockMeta` byte offsets are relative to the start of this
    /// buffer. Empty for inline (df=1) cursors, which never decode.
    /// Mirrors the vector reader's per-probed-cluster buffers: the
    /// search hot loops index only the bytes this term touches, never
    /// the whole postings region.
    ///
    /// Deliberately carries NO positional state: term cursors are the
    /// hot per-query unit the multi-cursor kernels iterate over, and
    /// the positional extras matter only to phrase members —
    /// [`PhraseMember`] re-derives them from these bytes instead, so
    /// plain term queries never pay for them in cursor or block-meta
    /// footprint.
    pub(super) bytes: Bytes,
    /// True when this term's FST slot carried no postings-length hint,
    /// so the build probed the 20-byte header before fetching the body
    /// — two planned byte-source ranges instead of one.
    pub(super) header_probed: bool,
    /// Count-only cursor: `decode_current_block` skips the tf half of each
    /// block (see [`decode_block_doc_ids`]). Set by the unranked count
    /// kernels (union / intersection), which never read `block_tfs`;
    /// leaves `block_tfs` stale, so a `count_only` cursor must not be used
    /// for scoring.
    pub(super) count_only: bool,
    /// Which block index is currently decoded into `block_doc_ids`
    /// (`usize::MAX` = none). Lets [`Self::contains`] skip re-decoding a
    /// PACKED block it already holds while probing membership across a
    /// run of ascending target docs.
    pub(super) decoded_block: usize,
    /// The posting-block codec this cursor's superfile uses (128-doc for
    /// `V1`–`V4`, 256-doc for `V5`). All block decode + bitset-header access
    /// routes through it.
    pub(super) codec: PostingCodec,
}

impl TermCursor {
    /// Parse one term's metadata + skip table out of its own postings
    /// byte range and decode its first block. `term_bytes` starts at
    /// the term's 20-byte metadata header (offset 0) and runs to the
    /// end of its last block — the contiguous range
    /// [`FtsReader::fetch_term_postings`] fetched for this term.
    pub(super) fn new(
        term_bytes: Bytes,
        n_docs: u64,
        positional: bool,
        global_idf: Option<f32>,
        header_probed: bool,
        count_only: bool,
        codec: PostingCodec,
    ) -> Result<Self, FtsError> {
        let postings: &[u8] = term_bytes.as_ref();
        let metadata_offset = 0usize;

        // The plain-term cursor never decodes positions, so it needs no
        // sub-index (it reads block offsets straight from the skip table).
        let term_meta = TermMeta::parse(
            postings,
            metadata_offset,
            positional,
            false,
            codec.subindex_entries_per_block(),
            codec.skip_entry_size(),
            codec.has_sub_block_bounds(),
        )?;
        let local_idf = bm25::idf(n_docs, term_meta.df);
        let idf = global_idf.unwrap_or(local_idf);
        // Stored per-block BMW upper bounds bake in the LOCAL idf. Only a
        // global-idf override needs to rescale them by global/local:
        // block_max = local_idf_x_k1p1 × (an idf-independent tf-factor),
        // so the linear rescale is exact and keeps the BMW skip UBs
        // consistent with the global-idf scores computed from
        // `idf_x_k1p1` below. `None` (the default per-superfile path, and
        // the case where a gathered global idf happens to equal the
        // local one) leaves the stored value untouched — the block loop
        // does no extra work, matching the per-superfile scorer exactly.
        let idf_rescale = match global_idf {
            Some(_) if local_idf > 0.0 && idf != local_idf => Some(idf / local_idf),
            _ => None,
        };

        // Collect straight into the `Arc` allocation: `0..num_blocks` is
        // an exact-size iterator, so this writes each entry in place —
        // one allocation, no intermediate `Vec` + copy. The skip table
        // is ~a quarter of a long term's cursor-build bytes (one 32-byte
        // entry per 128-doc block), so the doubled write showed up on
        // common-term queries.
        let mut term_max_bm25: f32 = 0.0;
        let blocks: Arc<[BlockMeta]> = (0..term_meta.num_blocks)
            .map(|i| {
                let (last_doc_id, block_offset_in_term, raw_lo, raw_hi, mid_last_doc_id) =
                    term_meta.skip_entry(postings, i);
                let rescale = |v: f32| match idf_rescale {
                    Some(ratio) => v * ratio,
                    None => v,
                };
                let block_max_bm25_lo = rescale(raw_lo);
                let block_max_bm25_hi = rescale(raw_hi);
                term_max_bm25 = term_max_bm25.max(block_max_bm25_lo).max(block_max_bm25_hi);

                BlockMeta {
                    last_doc_id,
                    block_byte_offset: metadata_offset + block_offset_in_term,
                    block_byte_end: metadata_offset + term_meta.block_end_in_term(postings, i),
                    block_max_bm25_lo,
                    block_max_bm25_hi,
                    mid_last_doc_id,
                }
            })
            .collect();

        let mut cursor = Self {
            idf_x_k1p1: idf * (bm25::K1 + 1.0),
            term_max_bm25,
            df: term_meta.df,
            blocks,
            block_doc_ids: vec![0u32; BLOCK_LEN_MAX],
            block_tfs: vec![0u32; BLOCK_LEN_MAX],
            block_n: 0,
            current_block: 0,
            pos: 0,
            inspect_block: 0,
            inspect_target: 0,
            bytes: term_bytes,
            header_probed,
            count_only,
            decoded_block: usize::MAX,
            codec,
        };
        if !cursor.blocks.is_empty() {
            cursor.decode_current_block();
        }
        Ok(cursor)
    }

    /// Synthesize a cursor for a df=1 inline-encoded term. Skips the
    /// postings-region read entirely — the caller already has
    /// (doc_id, tf) from unpacking the FST value, and BMW upper bound
    /// for a 1-doc term equals that doc's actual BM25 score (only one
    /// doc means min_dl = dl and max_tf = tf, so the per-block UB
    /// formula collapses to the score itself). Computed at query time
    /// since there's no skip-table entry stored for inline terms.
    pub(super) fn new_inline(
        doc_id: u32,
        tf: u32,
        n_docs: u64,
        dl_norm_k1: f32,
        global_idf: Option<f32>,
        codec: PostingCodec,
    ) -> Self {
        let idf = global_idf.unwrap_or_else(|| bm25::idf(n_docs, 1));
        let idf_x_k1p1 = idf * (bm25::K1 + 1.0);
        let block_max_bm25 = bm25::score_with_dl_norm_k1(idf_x_k1p1, tf, dl_norm_k1);

        let blocks: Arc<[BlockMeta]> = Arc::from([BlockMeta {
            last_doc_id: doc_id,
            // No postings-region bytes back this cursor; the decoded
            // buffer is pre-filled below so `decode_current_block` is
            // never called against these offsets.
            block_byte_offset: 0,
            block_byte_end: 0,
            // One doc: no second half, and the whole-block bound is this doc's
            // exact score. mid == doc_id so only the first half is ever chosen.
            block_max_bm25_lo: block_max_bm25,
            block_max_bm25_hi: block_max_bm25,
            mid_last_doc_id: doc_id,
        }]);

        let mut block_doc_ids = vec![0u32; BLOCK_LEN_MAX];
        let mut block_tfs = vec![0u32; BLOCK_LEN_MAX];
        block_doc_ids[0] = doc_id;
        block_tfs[0] = tf;

        Self {
            idf_x_k1p1,
            term_max_bm25: block_max_bm25,
            df: 1,
            blocks,
            block_doc_ids,
            block_tfs,
            block_n: 1,
            current_block: 0,
            pos: 0,
            inspect_block: 0,
            inspect_target: 0,
            bytes: Bytes::new(),
            header_probed: false,
            // Inline cursors carry their single posting pre-decoded and
            // never call `decode_current_block`, so the flag is inert.
            count_only: false,
            decoded_block: 0,
            // Inert: an inline cursor never decodes a block.
            codec,
        }
    }

    pub(super) fn decode_current_block(&mut self) {
        let codec = self.codec;
        let block = self.blocks[self.current_block];
        // Borrow in place rather than clone an owned `Bytes` (disjoint from the
        // `&mut self.block_*` decode targets, which are separate fields).
        let bytes = &self.bytes[block.block_byte_offset..block.block_byte_end];
        // Count-only cursors skip the tf half of the block; the count
        // kernels never read `block_tfs`, so it is left stale.
        self.block_n = match self.count_only {
            true => codec.decode_block_doc_ids(bytes, &mut self.block_doc_ids),
            false => codec.decode_block(bytes, &mut self.block_doc_ids, &mut self.block_tfs),
        };
        self.pos = 0;
        self.decoded_block = self.current_block;
    }

    /// Membership probe: does this term contain `doc`? Advances the block
    /// cursor forward to the block that could hold `doc` (targets arrive
    /// ascending on the AND-count leapfrog) and, on a **bitset block**,
    /// answers with a single bit-test — no decode. A PACKED block is
    /// decoded once (cached via `decoded_block`) and binary-searched. Used
    /// only by the count leapfrog; it moves `current_block`, so a cursor
    /// probed with `contains` must not also be iterated.
    pub(super) fn contains(&mut self, doc: u32) -> bool {
        while self.current_block < self.blocks.len()
            && self.blocks[self.current_block].last_doc_id < doc
        {
            self.current_block += 1;
        }
        if self.current_block >= self.blocks.len() {
            return false;
        }
        // Inline (df=1) cursor: single pre-decoded doc, no postings bytes.
        if self.bytes.is_empty() {
            return self.block_n > 0 && self.block_doc_ids[0] == doc;
        }
        let block = self.blocks[self.current_block];
        // Borrow the block's bytes in place — `self.bytes` is held for the
        // cursor's life, so a subslice needs no owned `Bytes` clone. A
        // per-probe `.slice()` here bumps and drops an atomic refcount on
        // every membership probe; over a long driver it was ~11% of the
        // intersection-count time (and wasted on the PACKED path, which
        // only reads the encoding byte before falling to the decode cache).
        let codec = self.codec;
        let raw = &self.bytes[block.block_byte_offset..block.block_byte_end];
        // `BASE_OFF` (4) and `HEADER_SIZE` (8) are identical across codecs; only
        // the `encoding`/`tf_bits` offsets and the block length differ.
        if raw[codec.encoding_off()] == codec.encoding_bitset() {
            let base = read_u32_le(&raw[block256::BASE_OFF..block256::BASE_OFF + 4]);
            if doc < base {
                return false;
            }
            let bit = (doc - base) as usize;
            let tfs_size = codec.block_len() * raw[codec.tf_bits_off()] as usize / 8;
            let bitset_end = raw.len() - tfs_size;
            let word_at = block256::HEADER_SIZE + (bit / 64) * 8;
            if word_at + 8 > bitset_end {
                return false; // past this block's presence bits ⇒ absent
            }
            let word = u64::from_le_bytes(raw[word_at..word_at + 8].try_into().expect("8 bytes"));
            (word >> (bit % 64)) & 1 == 1
        } else {
            // Borrow of `raw` ends above; the decode needs `&mut self`.
            if self.decoded_block != self.current_block {
                self.decode_current_block();
            }
            self.block_doc_ids[..self.block_n]
                .binary_search(&doc)
                .is_ok()
        }
    }

    /// Materialize a `contains`-probed cursor at `doc`: ensure the current
    /// block is decoded and `pos` points at `doc`. A membership probe
    /// (`contains`) advances `current_block` but, on a **bitset block**,
    /// answers by bit-test without decoding — leaving `block_doc_ids`,
    /// `block_tfs`, and `pos` stale. The phrase position-verification path
    /// needs the fully decoded block; this decodes it (only when the current
    /// block isn't already decoded) and scans `pos` up to `doc`. Callers
    /// pass a `doc` a preceding `contains(doc)` confirmed is present, arriving
    /// in ascending order, so the forward `pos` scan always lands on it.
    pub(super) fn materialize_at(&mut self, doc: u32) {
        if self.decoded_block != self.current_block {
            self.decode_current_block();
        }
        while self.pos < self.block_n && self.block_doc_ids[self.pos] < doc {
            self.pos += 1;
        }
    }

    pub(super) fn is_exhausted(&self) -> bool {
        self.current_block >= self.blocks.len()
    }

    /// Block count, used as a cheap proxy for df when AND intersection
    /// picks the rarest cursor as the leader. Block count is an exact
    /// upper bound on df: a term's df is `(blocks - 1) * BLOCK_LEN +
    /// last_block_n`, so cursors compare in the same order by block
    /// count as they do by df. Inline cursors return 1.
    #[inline(always)]
    pub(super) fn block_count(&self) -> usize {
        self.blocks.len()
    }

    #[inline(always)]
    pub(super) fn current_doc_id(&self) -> u32 {
        if self.is_exhausted() || self.pos >= self.block_n {
            u32::MAX
        } else {
            self.block_doc_ids[self.pos]
        }
    }

    #[inline(always)]
    pub(super) fn current_tf(&self) -> u32 {
        debug_assert!(!self.is_exhausted() && self.pos < self.block_n);
        self.block_tfs[self.pos]
    }

    /// The 128-doc half of the current block the position sits in, as
    /// `(half BM25 upper bound, half's last doc id)`. Ranked block skips use
    /// this so pruning re-pivots at 128 granularity even though blocks decode
    /// 256-wide: the stored per-half bounds (`lo`/`hi`, split at
    /// `mid_last_doc_id`) make the tighter bound safe. When the position has
    /// run off the decoded block it falls back to the whole-block bound / end.
    /// On pre-V5 blobs the two halves are equal, so this is the whole block.
    #[inline(always)]
    fn current_half_bound(&self) -> (f32, u32) {
        let b = &self.blocks[self.current_block];
        if self.pos >= self.block_n {
            (b.block_max_bm25(), b.last_doc_id)
        } else if self.block_doc_ids[self.pos] <= b.mid_last_doc_id {
            (b.block_max_bm25_lo, b.mid_last_doc_id)
        } else {
            (b.block_max_bm25_hi, b.last_doc_id)
        }
    }

    #[inline(always)]
    pub(super) fn current_block_max_bm25(&self) -> f32 {
        if self.is_exhausted() {
            0.0
        } else {
            self.current_half_bound().0
        }
    }

    /// Last doc_id of the current block's active 128-doc half — the split
    /// point when the position is in the first half, else the block's last
    /// doc. Paired with `current_block_max_bm25` as the block-skip window so
    /// the BMW/MaxScore walk computes its "next interesting doc_id" at the
    /// half boundary and re-pivots there. On pre-V5 blobs mid == last, so this
    /// is the whole-block last doc.
    #[inline(always)]
    pub(super) fn current_block_last_doc_id(&self) -> u32 {
        if self.is_exhausted() {
            u32::MAX
        } else {
            self.current_half_bound().1
        }
    }

    /// Shallow-advance the inspect-block pointer to the block that
    /// would contain `target`. Does NOT decode and does NOT touch the
    /// doc cursor (`current_block`, `pos`, decoded buffers stay put);
    /// only the lightweight `inspect_block` index moves. Used by the
    /// BMW UB sum at `pivot_doc` for cursors whose current_doc lags
    /// pivot_doc — their relevant block-max is the block containing
    /// pivot_doc, not their current decoded block.
    ///
    /// Monotonically advances; calling this for monotonically-
    /// increasing `target` across WAND iterations gives amortized
    /// O(1) per call.
    pub(super) fn shallow_advance_block_to(&mut self, target: u32) {
        // Remember the target so `inspect_block_*` report the bound for the
        // 128-doc half that contains it, not the whole 256-doc block.
        self.inspect_target = target;
        // Never let inspect_block fall behind current_block — once
        // the doc cursor has decoded past a block, that block's
        // metadata is no longer relevant.
        if self.inspect_block < self.current_block {
            self.inspect_block = self.current_block;
        }
        while self.inspect_block < self.blocks.len()
            && self.blocks[self.inspect_block].last_doc_id < target
        {
            self.inspect_block += 1;
        }
    }

    /// Maximum `block_max_bm25` across all blocks of this cursor whose
    /// doc-id range overlaps `[range_start, range_end]` (inclusive on
    /// both ends). Used by AND block-max pruning to compute a safe
    /// upper bound on this cursor's contribution across the leader's
    /// current block — a single-block lookup at one boundary
    /// underestimates when the leader's range spans multiple
    /// cursor blocks with varying block_max. Uses `inspect_block` as
    /// a hint pointer so monotonically-advancing leader ranges amortize
    /// to O(1) amortized per call.
    pub(super) fn block_max_in_range(&mut self, range_start: u32, range_end: u32) -> f32 {
        // Advance inspect_block to the first block whose last_doc_id
        // could intersect the range. shallow_advance_block_to lands on
        // the first block with last_doc_id >= range_start, which is
        // exactly the first block that can overlap the range.
        self.shallow_advance_block_to(range_start);
        let mut max: f32 = 0.0;
        let mut i = self.inspect_block;
        while i < self.blocks.len() {
            // Block i starts at the doc right after the previous block's
            // last_doc_id (or doc 0 if i == 0). Once block_start exceeds
            // range_end the rest of the blocks lie strictly past the
            // range; stop walking.
            let block_start = if i == 0 {
                0u32
            } else {
                self.blocks[i - 1].last_doc_id.saturating_add(1)
            };
            if block_start > range_end {
                break;
            }
            // Include only the 128-doc half(s) the range actually overlaps:
            // the first half spans [block_start, mid], the second (mid, last].
            // For a single-doc range this selects exactly the half holding the
            // doc, so the returned bound is 128-granular. On pre-V5 blobs
            // mid == last, so only the (equal) first-half bound is ever taken.
            let b = &self.blocks[i];
            if range_start <= b.mid_last_doc_id {
                max = max.max(b.block_max_bm25_lo);
            }
            if range_end > b.mid_last_doc_id {
                max = max.max(b.block_max_bm25_hi);
            }
            i += 1;
        }
        max
    }

    /// Block-max-BM25 at the inspect-block pointer, for the 128-doc half that
    /// contains the last-shallow-advanced target. Pair with
    /// `shallow_advance_block_to(pivot_doc)` to bound the cursor's contribution
    /// at pivot_doc. Reporting the relevant half (not the whole 256-doc block)
    /// keeps ranked pruning as tight as the 128-doc layout. On pre-V5 blobs the
    /// two halves are equal, so this is the whole-block bound.
    pub(super) fn inspect_block_max_bm25(&self) -> f32 {
        if self.inspect_block >= self.blocks.len() {
            0.0
        } else {
            let b = &self.blocks[self.inspect_block];
            if self.inspect_target <= b.mid_last_doc_id {
                b.block_max_bm25_lo
            } else {
                b.block_max_bm25_hi
            }
        }
    }

    /// Last doc_id of the inspect pointer's current half — the first half's
    /// `mid_last_doc_id` when the target lands in it, else the block end. Used
    /// as the BMW skip window end: bounding to the half (not the whole 256-doc
    /// block) lets the walk re-pivot at the half boundary. On pre-V5 blobs mid
    /// == last_doc_id, so this is the whole-block end.
    pub(super) fn inspect_block_last_doc_id(&self) -> u32 {
        if self.inspect_block >= self.blocks.len() {
            u32::MAX
        } else {
            let b = &self.blocks[self.inspect_block];
            if self.inspect_target <= b.mid_last_doc_id {
                b.mid_last_doc_id
            } else {
                b.last_doc_id
            }
        }
    }

    /// Advance one position. Crosses block boundaries automatically;
    /// decodes the next block on demand.
    #[inline(always)]
    pub(super) fn next(&mut self) {
        if self.is_exhausted() {
            return;
        }
        self.pos += 1;
        if self.pos >= self.block_n {
            self.advance_block();
        }
    }

    /// Advance a known in-block batch, crossing to the next block when
    /// `count` consumes its remaining postings. Unlike [`Self::next`],
    /// callers must not start at or advance past the decoded block end.
    #[inline(always)]
    pub(super) fn advance_by(&mut self, count: usize) {
        debug_assert!(!self.is_exhausted());
        debug_assert!(count > 0 && self.pos + count <= self.block_n);
        self.pos += count;
        // The assertion above makes equality equivalent to `>=` here.
        if self.pos == self.block_n {
            self.advance_block();
        }
    }

    /// Move to and decode the next posting block, or mark the cursor
    /// exhausted when the current block is the last one.
    #[inline(always)]
    pub(super) fn advance_block(&mut self) {
        self.current_block += 1;
        if self.current_block > self.inspect_block {
            self.inspect_block = self.current_block;
        }
        if self.current_block < self.blocks.len() {
            self.decode_current_block();
        }
    }

    /// Skip forward so `current_doc_id() >= target`. Uses the skip
    /// table to skip whole blocks when the entire block precedes
    /// `target`. Common-case fast path (target lies within the
    /// already-decoded current block) is just an inlined `pos++`
    /// scan — no re-decode, no `is_exhausted` rechecks.
    #[inline(always)]
    pub(super) fn skip_to(&mut self, target: u32) {
        if self.is_exhausted() {
            return;
        }
        let cur_block = self.current_block;
        let cur_block_last = self.blocks[cur_block].last_doc_id;
        if cur_block_last >= target {
            // Fast path: target is in our currently-decoded block.
            // Just scan pos forward. The `current_doc_id() >= target`
            // guard from before is folded into this scan — if pos is
            // already at-or-past, the loop body doesn't execute.
            let n = self.block_n;
            while self.pos < n && self.block_doc_ids[self.pos] < target {
                self.pos += 1;
            }
            if self.pos < n {
                return;
            }
            // Walked off the end of the decoded block (rare under
            // skip-table invariants); fall through to cross-block.
        }
        self.skip_to_cross_block(target);
    }

    /// Cross-block path of `skip_to`: target is past the current
    /// decoded block. Advances `current_block` via the skip table,
    /// decodes the new block (only when crossing), and scans pos.
    /// Pulled out so the within-block fast path stays small enough
    /// to inline at every call site.
    #[cold]
    pub(super) fn skip_to_cross_block(&mut self, target: u32) {
        while self.current_block < self.blocks.len()
            && self.blocks[self.current_block].last_doc_id < target
        {
            self.current_block += 1;
        }
        if self.current_block > self.inspect_block {
            self.inspect_block = self.current_block;
        }
        if self.is_exhausted() {
            return;
        }
        self.decode_current_block();
        while self.pos < self.block_n && self.block_doc_ids[self.pos] < target {
            self.pos += 1;
        }
        if self.pos >= self.block_n {
            self.current_block += 1;
            if self.current_block > self.inspect_block {
                self.inspect_block = self.current_block;
            }
            if self.current_block < self.blocks.len() {
                self.decode_current_block();
            }
        }
    }
}

#[cfg(test)]
mod codec_tests {
    use super::PostingCodec;
    use crate::superfile::format;

    #[test]
    fn from_version_selects_block_size_and_subindex_stride() {
        // V1–V4 → the 128-doc codec; V5 → the 256-doc codec.
        for v in [
            format::fts::VERSION_V1_LEGACY,
            format::fts::VERSION_V2,
            format::fts::VERSION_V3,
            format::fts::VERSION_V4,
        ] {
            assert_eq!(PostingCodec::from_version(v), PostingCodec::Block128);
            assert_eq!(PostingCodec::from_version(v).block_len(), 128);
        }
        assert_eq!(
            PostingCodec::from_version(format::fts::VERSION_V5),
            PostingCodec::Block256
        );
        assert_eq!(
            PostingCodec::from_version(format::fts::VERSION_V5).block_len(),
            256
        );
        // Header-offset differences: encoding byte 3→2, tf_bits 2→1.
        assert_eq!(PostingCodec::Block128.encoding_off(), 3);
        assert_eq!(PostingCodec::Block256.encoding_off(), 2);
        assert_eq!(PostingCodec::Block128.tf_bits_off(), 2);
        assert_eq!(PostingCodec::Block256.tf_bits_off(), 1);
        // Sub-index entries per block scale with the block size.
        assert_eq!(PostingCodec::Block128.subindex_entries_per_block(), 8);
        assert_eq!(PostingCodec::Block256.subindex_entries_per_block(), 16);
        // V5 skip entries carry the two 128-half sub-block bound fields (+8 bytes).
        assert_eq!(PostingCodec::Block128.skip_entry_size(), 16);
        assert_eq!(PostingCodec::Block256.skip_entry_size(), 24);
        assert!(!PostingCodec::Block128.has_sub_block_bounds());
        assert!(PostingCodec::Block256.has_sub_block_bounds());
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use bytes::Bytes;

    use crate::superfile::fts::{
        bm25, builder::FtsBuilder, reader::FtsReader, tokenize::AsciiLowerTokenizer,
    };

    /// The per-block BM25 upper bound stored in the skip table must be a
    /// valid upper bound over the *query-time* score of every document in
    /// that block. Query-time scoring reads each document's length from the
    /// byte-quantized norm table, which truncates the length downward — and
    /// a shorter length yields a *higher* BM25 score. If the stored block
    /// max is computed from the exact (un-truncated) length, it lands below
    /// the query score of a doc whose length quantizes down, and the
    /// block-max skip in the ranked-OR walk drops that doc from the top-k.
    ///
    /// This plants a term spanning several 128-doc blocks whose documents
    /// all have a length in the quantize-down region, then walks the term's
    /// cursor and asserts `block_max >= query_score` for every posting.
    /// Without the length-consistent block bound the assertion fires on the
    /// highest-tf doc in each block; a small-doc corpus (every length in the
    /// exact-quantization region) never exercises it.
    #[tokio::test]
    async fn block_max_bounds_query_time_score() {
        // A length that truncates under the one-byte length quantizer:
        // `dequantize_len(quantize_len(200)) == 192`, so a length-200 doc is
        // scored as if length 192 and scores *higher* than at its true
        // length.
        const DOC_LEN: usize = 200;
        assert!(
            bm25::dequantize_len(bm25::quantize_len(DOC_LEN as u32)) < DOC_LEN as u32,
            "corpus doc length must quantize downward to exercise the bound"
        );
        // The term under test lives in this many docs — enough to span
        // multiple 128-doc blocks so the block-max skip engages.
        const TERM_DOCS: u32 = 260;
        // Total corpus size. Kept well above `TERM_DOCS` so the term's IDF
        // is large enough that the quantization-induced score gap clears the
        // skip table's fixed-point rounding and the assertion is decisive.
        const N_DOCS: u32 = 1300;

        let tok = Arc::new(AsciiLowerTokenizer);
        let mut b = FtsBuilder::new(tok);
        b.register_column("body".into(), false)
            .expect("register column");
        for doc_id in 0..N_DOCS {
            // Every doc is `DOC_LEN` tokens long (so `avgdl == DOC_LEN` and
            // every length quantizes down identically). The term docs carry
            // `common` with a term frequency of 1..=3 — a genuine per-block
            // spread of scores whose maximum is the highest-tf doc — padded
            // with a filler token; the rest are filler only.
            let common_tf = if doc_id < TERM_DOCS {
                1 + (doc_id % 3) as usize
            } else {
                0
            };
            let mut text = String::with_capacity(DOC_LEN * 5);
            for _ in 0..common_tf {
                text.push_str("common ");
            }
            for _ in 0..(DOC_LEN - common_tf) {
                text.push_str("pad ");
            }
            b.add_doc(0, doc_id, text.trim_end()).expect("add doc");
        }
        let bytes = Bytes::from(b.finish().expect("finish builder"));
        let json = r#"[{"name":"body","tokenizer":"ascii_lower"}]"#;
        let reader = FtsReader::open(bytes, json).expect("open FtsReader");

        let mut cursors = reader
            .build_term_cursors(0, &["common"], None, false)
            .await
            .expect("build term cursors");
        let cursor = cursors.first_mut().expect("`common` present in dictionary");
        assert!(
            cursor.blocks.len() >= 2,
            "term must span multiple blocks so the block-max skip engages \
             (got {} block(s))",
            cursor.blocks.len()
        );
        let col_meta = &reader.columns[0];

        let mut checked = 0u32;
        while !cursor.is_exhausted() {
            let doc = cursor.current_doc_id();
            let tf = cursor.current_tf();
            let query_score =
                bm25::score_with_dl_norm_k1(cursor.idf_x_k1p1, tf, col_meta.dl_norm_k1.get(doc));
            let block_max = cursor.current_block_max_bm25();
            assert!(
                block_max >= query_score,
                "stored block max {block_max} < query-time score {query_score} for \
                 doc {doc} (tf={tf}): the per-block BM25 bound under-estimates a \
                 document in its own block, so the ranked-OR block-max skip can drop it",
            );
            checked += 1;
            cursor.next();
        }
        assert_eq!(
            checked, TERM_DOCS,
            "every posting for the term must be visited"
        );
    }
}
