use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs
{
    #[arg(short, long, default_value = "settings/config.yaml")]
    pub config_path: String,
}

pub fn parse() -> CliArgs
{
    CliArgs::parse()
}
