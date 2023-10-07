pub fn set_up_ctrlc() -> std::sync::Arc<std::sync::atomic::AtomicBool> {
    let running = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, std::sync::atomic::Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");
    running
}

// the use of this variable is rly strict so it being static mut shouldn't cause any problems
static mut LAST_LOOP_TIME: Option<std::time::Instant> = None;

pub fn check_loop_health() {
    let llt_opt = unsafe { LAST_LOOP_TIME };
    if llt_opt.is_none() {
        debug!("LLT is none, setting to atm");
        unsafe {
            LAST_LOOP_TIME = Some(std::time::Instant::now());
        };
        return;
    }

    let llt = llt_opt.unwrap();

    let max_loop_time = 1. / crate::TARGET_TPS;

    let elapsed = llt.elapsed();
    if elapsed > std::time::Duration::from_secs_f32(max_loop_time * 1.1) {
        warn!(
            "[Server] Main loop failled to run at 10TPS: +{:.3?}",
            elapsed - std::time::Duration::from_secs_f32(max_loop_time)
        );
    }
    unsafe { LAST_LOOP_TIME = Some(std::time::Instant::now()) }
}
