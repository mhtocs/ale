use fern::colors::{Color, ColoredLevelConfig};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::panic;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Devtool for eventloganalyzer , pass `-h` for more info")]
pub struct Opt {
    #[structopt(flatten)]
    pub config: FilePath,
}

#[derive(Debug, StructOpt)]
pub struct FilePath {
    #[structopt(short, long, parse(from_os_str))]
    filepath: Option<PathBuf>,
}

impl FilePath {
    pub fn with_default<T: Into<PathBuf>>(&self, default: T) -> PathBuf {
        match &self.filepath {
            Some(x) => x.clone(),
            None => default.into(),
        }
    }
}

impl fmt::Display for FilePath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.filepath {
            Some(x) => write!(f, "{}", x.to_str().unwrap_or("bad filename")),
            None => write!(f, "None"),
        }
    }
}

impl Opt {
    pub fn from_args() -> Self {
        StructOpt::from_args()
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Config {
    pub procs: Vec<Proc>,
    pub max_retry: i32,
    pub sleep_delay: u64,
    pub server_url: String,
    pub es_url: String,
    pub loglevel: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Proc {
    pub pid: i32,
    pub name: String,
    pub path: Option<PathBuf>,
}

const PKG_NAME: &'static str = env!("CARGO_PKG_NAME");

pub fn init_logger(loglevel: &str) -> Result<(), fern::InitError> {
    let colors_line = ColoredLevelConfig::new()
        .error(Color::BrightRed)
        .warn(Color::Yellow)
        .info(Color::BrightGreen)
        .debug(Color::BrightYellow)
        .trace(Color::BrightBlue);

    // let colors_level = colors_line
    //     .clone()
    //     .info(Color::BrightGreen)
    //     .debug(Color::BrightBlue);

    fern::Dispatch::new()
        .level(match loglevel {
            "info" => log::LevelFilter::Info,
            "error" => log::LevelFilter::Error,
            "debug" => log::LevelFilter::Debug,
            _ => log::LevelFilter::Trace,
        })
        .chain(
            fern::Dispatch::new()
                .format(move |out, message, record| {
                    out.finish(format_args!(
                        "{color_line}[{level} {date} {target}:{line_no}]\x1B[0m {message}",
                        color_line = format_args!(
                            "\x1B[{}m",
                            colors_line.get_color(&record.level()).to_fg_str()
                        ),
                        level = record.level(), //colors_level.color(record.level()),
                        date = chrono::Local::now().format("%Y%m%d %H:%M:%S"),
                        target = record.target(),
                        line_no = record.line().unwrap_or(0),
                        message = message,
                    ));
                })
                .chain(std::io::stdout()),
        )
        .chain(
            fern::Dispatch::new()
                .format(move |out, message, record| {
                    out.finish(format_args!(
                        "[{level} {date} {target}] {message}\x1B[0m",
                        level = record.level(),
                        date = chrono::Local::now().format("%y%m%d %H:%M:%S"),
                        target = record.target(),
                        message = message,
                    ));
                })
                .chain(fern::log_file(format!("{}.log", PKG_NAME))?),
        )
        .apply()?;

    panic::set_hook(Box::new(|info| {
        log::error!("it panicked :: {}", info);
    }));
    Ok(())
}

pub fn set_ctrlc() {
    ctrlc::set_handler(move || {
        println!("received Ctrl+C!");
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");
}
