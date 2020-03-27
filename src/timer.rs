use std::thread;
use std::time::{Instant, Duration};
use std::sync::atomic::{AtomicU64, AtomicBool, Ordering};

/// Milliseconds since ANCHOR.
static RECENT: AtomicU64 = AtomicU64::new(0);
lazy_static! {
    static ref ANCHOR: Instant = Instant::now();
}

/// Returns milliseconds since ANCHOR.
/// 
/// ANCHOR is some fixed point in history.
pub fn now_millis() -> u64 {
    let res = Instant::now();
    let mut t = res.checked_duration_since(*ANCHOR).map_or(0, |d| d.as_secs() * 1000 + d.subsec_millis() as u64);
    let mut recent = RECENT.load(Ordering::Relaxed);
    loop {
        if recent > t {
            t = recent;
            break;
        }
        match RECENT.compare_exchange_weak(recent, t, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(_) => break,
            Err(r) => recent = r,
        }
    }
    RECENT.store(t, Ordering::Relaxed);
    t
}

/// Returns recent returned value by `now_millis`.
pub fn recent_millis() -> u64 {
    RECENT.load(Ordering::Relaxed)
}

lazy_static! {
    static ref UPDATER_IS_RUNNING: AtomicBool = AtomicBool::new(false);
}

/// Ensures background updater is running, which will call `now_millis` periodically.
pub(crate) fn ensure_updater() {
    if !UPDATER_IS_RUNNING.compare_and_swap(false, true, Ordering::SeqCst) {
        std::thread::Builder::new().name("time updater".to_owned()).spawn(|| {
            loop {
                thread::sleep(Duration::from_millis(200));
                now_millis();
            }
        }).unwrap();
    }
}