use shared::prelude::*;
pub mod utils;

const TARGET_TPS: f32 = 10.;

fn main() {
    shared::logger::init(Some("server.log"));
    let stopwatch = shared::time::Stopwatch::start_new();

    let mut loop_helper = spin_sleep::LoopHelper::builder()
        .report_interval_s(0.5)
        .build_with_target_rate(TARGET_TPS);

    let running = utils::set_up_ctrlc();

    debug!("Starting loop with {TARGET_TPS}TPS");

    while running.load(std::sync::atomic::Ordering::SeqCst) {
        loop_helper.loop_start();

        loop_helper.loop_sleep();
    }

    // spin_sleep::sleep(std::time::Duration::from_secs_f32(1.5));

    debug!(
        "Stopping loop. The server ran {}",
        shared::time::display_duration(stopwatch.read(), "")
    );
}
