use color_eyre::eyre::{eyre, Result, WrapErr};
use digital_garden::write;
use directories::UserDirs;
use std::path::PathBuf;
use structopt::StructOpt;

/// A CLI for the growing and curation of a digital garden
/// by https://daz.dev via https://egghead.io
#[derive(StructOpt, Debug)]
#[structopt(name = "garden")]
struct Opt {
  #[structopt(
    parse(from_os_str),
    short = "p",
    long,
    env
  )]
  garden_path: Option<PathBuf>,

  #[structopt(subcommand)]
  cmd: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
  /// Write something in your digital garden
  ///
  /// This command will open your $EDITOR, wait for you
  /// to write something, and then save the file to
  /// your digital garden
  Write {
    #[structopt(short, long)]
    title: Option<String>,
  },
}

fn get_default_garden_dir() -> Result<PathBuf> {
    let user_dirs = UserDirs::new().ok_or_else(|| {
        eyre!("Could not find home directory")
    })?;
    Ok(user_dirs.home_dir().join(".garden"))
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let opt = Opt::from_args();
    dbg!(&opt);
    let garden_path = match opt.garden_path {
        Some(pathbuf) => Ok(pathbuf),
        None => get_default_garden_dir()
            .wrap_err("`garden_path` was not supplied"),
    }?;
    match opt.cmd {
      Command::Write { title } => write( garden_path, title),
    }
}
