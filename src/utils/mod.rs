use chrono::Local;
use fern::Dispatch;

pub fn init_logger(release: bool) {
    let dispatch = Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} [{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                message
            ))
        })
        .chain(std::io::stdout());

    let dispatch = if release {
        dispatch
            .level(log::LevelFilter::Error)
            .chain(fern::log_file("output.log").unwrap())
    } else {
        dispatch.level(log::LevelFilter::Debug)
    };

    dispatch.apply().unwrap();
}
