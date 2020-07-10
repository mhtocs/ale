use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Opt {
    #[structopt(default_value = "..", long)]
    pub homepath: String,

    #[structopt(required = true, long)]
    pub es_pid: i32,

    #[structopt(required = true, long)]
    pub ela_pid: i32,
}

impl Opt {
    pub fn from_args() -> Self {
        StructOpt::from_args()
    }
}
