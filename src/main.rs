use std::io::Write;
use structopt::StructOpt;
use std::path::{Path, PathBuf};
use std::process;
use std::fs;

static PICO_HEADER: &str = "pico-8 cartridge // http://www.pico-8.com\nversion 16\n";
static HEADER_SEPARATOR: &str = "__lua__\n";
static FOOTER_SEPARATOR: &str = "__gfx__\n";

#[derive(Debug, StructOpt)]
struct Opt {
  #[structopt(short = "i", long = "input", parse(from_os_str), default_value = "src/")]
  input: PathBuf,
  #[structopt(parse(from_os_str))]
  output: Option<PathBuf>,
  #[structopt(short = "w", long = "watch")]
  watch: bool
}

fn error(message: String) {
  eprintln!("Error: {}", message);
  process::exit(1);
}

fn check_input_files(input: PathBuf) -> Vec<PathBuf> {
  if let Ok(metadata) = fs::metadata(&input) {
    if metadata.is_dir() {
      let mut sources: Vec<PathBuf> = Vec::new();
      let contents = input.read_dir().unwrap();
      for entry in contents {
        let path = entry.unwrap().path();
        if let Some(ext) = path.extension() {
          if ext.to_str().unwrap() == "lua" {
            sources.push(path);
          }
        }
      }
      if sources.len() == 0 {
        error(format!("No *.lua files found in the input directory ({:?})", input));
      }
      sources.sort();
      return sources;
    } else {
      error(format!("{:?} is a file, not a directory.", input));
    }
  } else {
    error(format!("{:?} needs to be a valid directory.", input));
  }
  panic!("Failed to validate input directory. Please report this bug.");
}

fn check_output_file(output: Option<PathBuf>) -> PathBuf {
  if let Some(path) = output {
    if let Ok(metadata) = fs::metadata(&path) {
      if metadata.is_file() {
        if let Some(ext) = path.extension() {
          if ext == "p8" {
            return path;
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
      return path;
    }
  } else {
    println!("Output name not specified, looking for a *.p8 file in the current directory..");
    let mut outputs: Vec<PathBuf> = Vec::new();
    let contents = fs::read_dir(".").unwrap();
    for entry in contents {
      let path = entry.unwrap().path();
      if let Some(ext) = path.extension() {
        if ext.to_str().unwrap() == "p8" {
          outputs.push(path);
        }
      }
    }
    if outputs.len() == 0 {
      let dir = Path::new(".").to_str().unwrap();
      let dir = format!("{}.p8", dir);
      println!("No *.p8 files found, generating a new one using the current directory's name ({})..", dir);
      fs::File::create(&dir).unwrap();
      return Path::new(&dir).to_path_buf();
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
  println!("Compiling {:?} into {:?}...", sources, output);
  let mut full_code = String::new();
  for file in sources {
    full_code.push_str(
      &fs::read_to_string(&file)
        .expect(&format!("Error: failed to read from file: {:?}", file))
    )
  }
  let mut f = fs::OpenOptions::new()
    .write(true)
    .truncate(true)
    .open(&output).unwrap();
  let mut new_content;
  let current_output = fs::read_to_string(&output)
    .expect(&format!("Error: failed to read from file: {:?}", output));
  if current_output != "" {
    let mut cartridge_iterator = current_output.split(HEADER_SEPARATOR);
    let header = cartridge_iterator
      .next()
      .expect("Error: failed to read the output file. The cartridge format seems to be incorrect.");
    let footer = cartridge_iterator
      .next()
      .expect("Error: failed to read the output file. The cartridge format seems to be incorrect.")
      .split(FOOTER_SEPARATOR)
      .skip(1)
      .next()
      .expect("Error: failed to read the output file. The cartridge format seems to be incorrect.");
    new_content = format!("{}{}{}{}{}",
      header,
      HEADER_SEPARATOR,
      full_code,
      FOOTER_SEPARATOR,
      footer
    );
  } else {
    new_content = format!("{}{}{}{}",
      PICO_HEADER,
      HEADER_SEPARATOR,
      full_code,
      FOOTER_SEPARATOR
    );
  }
  f.write_all(new_content.as_bytes()).expect("Error: failed to write to the output file.");
  f.sync_all().expect("Error: failed to write to the output file.");
}
