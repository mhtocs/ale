use std::panic;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "[devtool] YAMC for eventloganalyzer , pass `-h` for more info")]
pub struct Opt {
    #[structopt(default_value = "..", long)]
    pub homepath: String,

    #[structopt(required = true, long, help = "pid of elasticsearch")]
    pub es: i32,

    #[structopt(
        required = false,
        default_value = "0",
        long,
        name = "pid of ELA",
        help = "pid of evenloganalyzer"
    )]
    pub ela: i32,
}

impl Opt {
    pub fn from_args() -> Self {
        StructOpt::from_args()
    }
}

pub fn init_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "|{}|[{}]|[{}]| {}|",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;

    panic::set_hook(Box::new(|info| {
        log::error!("served panicked :: {}", info);
    }));
    Ok(())
}
