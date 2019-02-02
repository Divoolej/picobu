use structopt::StructOpt;
use std::path::{Path, PathBuf};
use std::process;
use std::fs;

#[derive(Debug, StructOpt)]
struct Opt {
  #[structopt(short = "i", long = "input", parse(from_os_str), default_value = "src/")]
  input: PathBuf,
  #[structopt(parse(from_os_str))]
  output: Option<PathBuf>
}

fn error(message: String) {
  eprintln!("Error: {}", message);
  process::exit(1);
}

fn check_input_files(input: PathBuf) -> Vec<String> {
  if let Ok(metadata) = fs::metadata(&input) {
    if metadata.is_dir() {
      let mut sources: Vec<String> = Vec::new();
      let contents = input.read_dir().unwrap();
      for entry in contents {
        let path = entry.unwrap().path();
        if let Some(ext) = path.extension() {
          if ext.to_str().unwrap() == "lua" {
            sources.push(path.file_name().unwrap().to_str().unwrap().to_string());
          }
        }
      }
      if sources.len() == 0 {
        error(format!("No *.p8 files found in the input directory ({:?})", input));
      }
      return sources;
    } else {
      error(format!("{:?} is a file, not a directory.", input));
    }
  } else {
    error(format!("{:?} needs to be a valid directory.", input));
  }
  panic!("Failed to validate input directory. Please report this bug.");
}
fn main() {
  let opt = Opt::from_args();
  let sources = check_input_files(opt.input);
}
