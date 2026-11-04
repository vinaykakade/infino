//! Experiment switches read once from the environment (default off).

use std::sync::OnceLock;

fn flag(cell: &'static OnceLock<bool>, name: &str) -> bool {
    *cell.get_or_init(|| std::env::var(name).is_ok_and(|v| v != "0"))
}

pub(crate) fn fast_str_minmax() -> bool {
    static C: OnceLock<bool> = OnceLock::new();
    flag(&C, "INFINO_EXP_FAST_STR_MINMAX")
}

pub(crate) fn counts_presize() -> bool {
    static C: OnceLock<bool> = OnceLock::new();
    flag(&C, "INFINO_EXP_COUNTS_PRESIZE")
}

pub(crate) fn stats_thread() -> bool {
    static C: OnceLock<bool> = OnceLock::new();
    flag(&C, "INFINO_EXP_STATS_THREAD")
}

pub(crate) fn copy_timing() -> bool {
    static C: OnceLock<bool> = OnceLock::new();
    flag(&C, "INFINO_EXP_COPY_TIMING")
}

/// Rows per decoded batch in the merge copy; 0 keeps the reader default.
pub(crate) fn decode_batch() -> usize {
    static C: OnceLock<usize> = OnceLock::new();
    *C.get_or_init(|| {
        std::env::var("INFINO_EXP_DECODE_BATCH")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(0)
    })
}

/// Thread-CPU nanoseconds spent in the body encoder's column tasks.
pub(crate) static ENCODE_TASK_CPU_NS: std::sync::atomic::AtomicU64 =
    std::sync::atomic::AtomicU64::new(0);

/// The calling thread's CPU clock when timing is on.
pub(crate) fn cpu_mark() -> Option<u128> {
    match copy_timing() {
        true => crate::runtime_metrics::cpu::thread_cpu_ns(),
        false => None,
    }
}

/// CPU nanoseconds since `mark` on this thread.
pub(crate) fn cpu_since(mark: Option<u128>) -> u64 {
    crate::runtime_metrics::cpu::thread_cpu_delta_ns(mark)
}
