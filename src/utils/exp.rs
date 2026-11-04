// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: Copyright The Infino Authors

//! Experiment switches read once from the environment, so one binary runs
//! both arms of an A/B. Every switch defaults to the existing behaviour.

use std::sync::OnceLock;

fn env_usize(name: &str) -> Option<usize> {
    std::env::var(name).ok().and_then(|v| v.parse().ok())
}

fn env_on(name: &str) -> bool {
    matches!(std::env::var(name).as_deref(), Ok("1" | "on" | "true"))
}

macro_rules! flag {
    ($fn:ident, $name:literal) => {
        pub(crate) fn $fn() -> bool {
            static V: OnceLock<bool> = OnceLock::new();
            *V.get_or_init(|| env_on($name))
        }
    };
}

macro_rules! count {
    ($fn:ident, $name:literal, $default:expr) => {
        pub(crate) fn $fn() -> usize {
            static V: OnceLock<usize> = OnceLock::new();
            *V.get_or_init(|| env_usize($name).unwrap_or($default).max(1))
        }
    };
}

// Open the merged superfiles' readers side by side when a compaction commit
// warms the reader cache.
flag!(par_warm, "INFINO_EXP_PAR_WARM");
// List the gc sweep's prefixes side by side.
flag!(gc_par_list, "INFINO_EXP_GC_PAR_LIST");
// Upload a term-index layer's objects as each key range finishes building.
flag!(ti_pipe, "INFINO_EXP_TI_PIPE");
// Seal re-stamps in flight at once.
count!(seal_writes, "INFINO_EXP_SEAL_WRITES", 64);
// Term-index object writes in flight at once, per kind.
count!(ti_writes, "INFINO_EXP_TI_WRITES", 64);
// Encode a sorted FTS merge's terms on the shared maintenance pool, whose
// threads the merges that have finished no longer use, rather than on the
// merge's own share of them.
flag!(encode_pool, "INFINO_EXP_ENCODE_POOL");
// Drop what a compaction commit is done with on a thread of its own.
flag!(bg_drop, "INFINO_EXP_BG_DROP");
// Gc deletes in flight at once.
count!(gc_deletes, "INFINO_EXP_GC_DELETES", 256);
// Count a compaction commit's own term-index objects as already checked.
flag!(ti_seen, "INFINO_EXP_TI_SEEN");
