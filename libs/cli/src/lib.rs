use anyhow::{Context, Result};
use clap::Parser;
use std::{io, path::PathBuf};
use youtube_dl::YoutubeDl;

struct Config {
    pingyin: bool,
    lang: String,
    dir: PathBuf,
}
trait Options {
    fn get_song();
    fn set_dir(path: String);
}

impl Options for Config {
    fn get_song() {
        todo!();
    }

    fn set_dir(path: String) {
        todo!()
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Play {
    song: String,

    #[arg(short, long, default_value_t = true)]
    romanized: bool,
}

pub fn get_input() -> Result<()> {
    let args = Play::parse();
    let path: PathBuf = ["~", "Desktop", "xvideos"].iter().collect();

    let config = Config {
        pingyin: true,
        lang: "en".to_string(),
        dir: path,
    };
    println!("{}", &args.romanized);

    let output = YoutubeDl::new(&args.song)
        .socket_timeout("15")
        .extract_audio(true)
        .output_directory(&config.dir.into_os_string().into_string().unwrap())
        .download(true)
        .run()
        .with_context(|| format!("Error reading URL: {}", &args.song))?;
    println!("{}", output.into_single_video().unwrap().title);
    Ok(())
}

fn print_output() -> Result<()> {
    let stdout = io::stdout();
    // aquire a lock on stdout and create a bufwriter on it
    let mut buf = io::BufWriter::new(stdout.lock());

    Ok(())
}
