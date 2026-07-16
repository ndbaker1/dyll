include!("dyll.rs");

#[ctor::ctor(unsafe)]
fn init() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

    log::debug!("--- MOCK INIT ---");

    print_hello::register_handler(&|print_hello| {
        log::info!("hello from mock!");
        print_hello();
    });
}
