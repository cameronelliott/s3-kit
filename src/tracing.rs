use tracing_subscriber::fmt::format::FmtSpan;

pub fn setup_tracing() {
    use tracing_subscriber::EnvFilter;

    // let env_filterx  = EnvFilter::builder()
    // .with_default_directive(LevelFilter::INFO.into())
    // .from_env_lossy();

    let env_filter = EnvFilter::from_default_env();
    //let enable_color = std::io::stdout().is_terminal(); // TODO
    let enable_color = atty::is(atty::Stream::Stdout);

    // let fmt_layer = tracing_subscriber::fmt::layer()
    //     .with_span_events(FmtSpan::ENTER)
    //     .with_target(true) // don't include event targets when logging
    //     .with_level(true); // don't include event levels when logging

    tracing_subscriber::fmt()
        // Use a more compact, abbreviated log format
        .compact()
        //  .pretty()
        .with_span_events(FmtSpan::ENTER)
        .with_env_filter(env_filter)
        .with_ansi(enable_color)
        .init();
}
