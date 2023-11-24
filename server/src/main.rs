#[macro_use]
extern crate log;
mod game_manager;
mod networking;
mod utils;
const TARGET_TPS: f32 = 10.;

fn main() {
    let config = logger::LoggerConfig::default().set_level(log::LevelFilter::Debug);

    logger::init(config, Some("./log/server.log"));

    let stopwatch = time::Stopwatch::start_new();

    let mut loop_helper = spin_sleep::LoopHelper::builder()
        .report_interval_s(0.5)
        .build_with_target_rate(TARGET_TPS);

    let running = utils::set_up_ctrlc();
    let mut server = networking::Server::<
        shared::message::ClientMessage,
        shared::message::ServerMessage,
    >::new(shared::DEFAULT_ADDRESS);

    let mut game_mgr = game_manager::GameManager::new();

    debug!("Starting loop with {TARGET_TPS}TPS");

    while running.load(std::sync::atomic::Ordering::SeqCst) {
        loop_helper.loop_start();
        utils::check_loop_health();

        server.update();

        game_mgr.update(&mut server);

        loop_helper.loop_sleep();
    }

    debug!(
        "Stopping loop. The server ran {}",
        time::display_duration(stopwatch.read())
    );
}
