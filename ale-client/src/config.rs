use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Opt {
    #[structopt(default_value = "..", long)]
    pub homepath: String,
}

impl Opt {
    pub fn from_args() -> Self {
        StructOpt::from_args()
    }
}
