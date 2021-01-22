use clap::Clap;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Clap)]
pub struct Opts {
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Clap)]
pub enum SubCommand {
    Download(DownloaderOpts),
}

#[derive(Clap)]
pub struct DownloaderOpts {
    #[clap(short, long)]
    pub out: PathBuf,
    #[clap(long)]
    pub only: Option<DownloaderSubCommand>,
}

#[derive(Clap)]
pub enum DownloaderSubCommand {
    #[clap(name = "browse")]
    DownloadBrowsePages,
    #[clap(name = "animes")]
    DownloadAnimePages,
}

impl FromStr for DownloaderSubCommand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "browse" => Ok(Self::DownloadBrowsePages),
            "anime" | "animes" => Ok(Self::DownloadAnimePages),
            _ => Err(format_err!("not a valid option")),
        }
    }
}
