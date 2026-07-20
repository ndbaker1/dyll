include!("dyll.rs");

#[ctor::ctor(unsafe)]
fn init() {
    tracing_subscriber::fmt::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    tracing::debug!("--- MOCK INIT ---");
    tracing::debug!(
        "command: {}",
        std::env::args().collect::<Vec<_>>().join(" ")
    );

    print_hello::register_handler(&|print_hello| {
        tracing::info!("hello from mock!");
        tracing::info!("calling original mock..");
        unsafe {
            print_hello();
        }
    });
}
