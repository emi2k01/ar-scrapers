use std::path::PathBuf;

use clap::Clap;

#[derive(Clap)]
pub struct Opts {
    #[clap(subcommand)]
    pub subcmd: SubCmd,
}

#[derive(Clap)]
pub enum SubCmd {
    Scrape,
    Render(RenderOpts),
}

#[derive(Clap)]
pub struct RenderOpts {
    #[clap(short, long)]
    pub out: PathBuf,
}
