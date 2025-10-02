use fern::colors::{Color, ColoredLevelConfig};
use std::time::SystemTime;

pub fn setup() {
    let colors = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        .info(Color::White)
        .debug(Color::White)
        .trace(Color::BrightBlack);
    let level = colors.info(Color::Green);

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{color_line}[{date} {level} {target} {color_line}] {message}\x1B[0m",
                color_line =
                    format_args!("\x1B[{}m", colors.get_color(&record.level()).to_fg_str()),
                date = humantime::format_rfc3339_seconds(SystemTime::now()),
                target = record.target(),
                level = level.color(record.level()),
                message = message,
            ))
        })
        .level(log::LevelFilter::Trace)
        .chain(std::io::stderr())
        .apply()
        .unwrap();
}
