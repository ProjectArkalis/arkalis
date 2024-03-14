use clap::Parser;

#[derive(Debug, Parser)]
pub struct Cli {
    #[arg(short, long, default_value = "./")]
    pub configs_path: String,
}
