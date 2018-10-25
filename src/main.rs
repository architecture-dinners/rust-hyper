extern crate rust_hyper;

// init slog out of band to make actual code logger agnostic
#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_envlogger;
extern crate slog_scope;
extern crate slog_stdlog;
extern crate slog_term;

fn main() -> Result<(), Box<::std::error::Error>> {
    // (max) when the guard is dropped, the global logger will reset. So
    // since rust_hyper::run() never returns...it will never reset :-)
    let _guard = init_logger();

    rust_hyper::run()
}

fn init_logger() -> slog_scope::GlobalLoggerGuard {
    use slog::Drain;

    let decorator = slog_term::TermDecorator::new().stderr().build();
    let fmt = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(slog_envlogger::new(fmt))
        .build()
        .fuse();
    let root_logger = slog::Logger::root(drain.fuse(), o!());

    slog_stdlog::init().unwrap();
    slog_scope::set_global_logger(root_logger)
}
