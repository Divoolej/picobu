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

fn check_output_file(output: Option<PathBuf>) -> String {
  if let Some(path) = output {
    if let Ok(metadata) = fs::metadata(&path) {
      if metadata.is_file() {
        if let Some(ext) = path.extension() {
          if ext == "p8" {
            return path.file_name().unwrap().to_str().unwrap().to_string();
          } else {
            error(format!("{:?} is not a valid *.p8 cartridge.", path));
          }
        } else {
          error("output file needs to have the .p8 extension.".to_string());
        }
      } else {
        error(format!("{:?} is not a valid *.p8 cartridge.", path));
      }
    } else {
      fs::File::create(&path).unwrap();
      return path.file_name().unwrap().to_str().unwrap().to_string();
    }
  } else {
    println!("Output name not specified, looking for a *.p8 file in the current directory..");
    let mut outputs: Vec<String> = Vec::new();
    let contents = fs::read_dir(".").unwrap();
    for entry in contents {
      let path = entry.unwrap().path();
      if let Some(ext) = path.extension() {
        if ext.to_str().unwrap() == "p8" {
          outputs.push(path.file_name().unwrap().to_str().unwrap().to_string());
        }
      }
    }
    if outputs.len() == 0 {
      let path = Path::new(".").canonicalize().unwrap();
      let dir = path.as_path().file_name().unwrap().to_str().unwrap().to_string();
      let dir = format!("{}.p8", dir);
      println!("No *.p8 files found, generating a new one using the current directory's name ({})..", dir);
      fs::File::create(&dir).unwrap();
      return dir;
    } else if outputs.len() == 1 {
      println!("Found a *.p8 file - {:?} will be used as the compilation output.", outputs[0]);
      return outputs.pop().unwrap();
    } else {
      error("Found more than one *.p8 file. Please specify the desired output in the arguments.".to_string());
    }
  }
  panic!("Failed to validate the output file. Please report this bug.");
}

fn main() {
  let opt = Opt::from_args();
  let sources = check_input_files(opt.input);
  let output = check_output_file(opt.output);
  println!("Compiling {:?} into {:?}", sources, output);
}
