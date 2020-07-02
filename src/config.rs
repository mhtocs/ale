use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Opt {
    #[structopt(default_value = "0.0.0.0", short, long)]
    pub host: String,

    #[structopt(default_value = "1337", short, long)]
    pub port: u16,
}

impl Opt {
    pub fn from_args() -> Self {
        StructOpt::from_args()
    }
}
