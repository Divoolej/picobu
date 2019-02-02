use hotwatch::{Hotwatch, Event};
use structopt::StructOpt;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, Arc};
use std::io::Write;
use std::process;
use std::thread;
use std::time;
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

fn check_input_files(input: &PathBuf) -> Vec<PathBuf> {
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
  panic!("Failed to validate input directory. Please report this issue.");
}

fn check_output_file(output: &Option<PathBuf>) -> PathBuf {
  if let Some(path) = output {
    if let Ok(metadata) = fs::metadata(&path) {
      if metadata.is_file() {
        if let Some(ext) = path.extension() {
          if ext == "p8" {
            return path.clone();
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
      return path.clone();
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
  panic!("Failed to validate the output file. Please report this issue.");
}

fn concatenate_sources(sources: &Vec<PathBuf>) -> String {
  let mut full_code = String::new();
  for file in sources {
    full_code.push_str(
      &fs::read_to_string(&file)
        .expect(&format!("Error: failed to read from file: {:?}", file))
    )
  }
  full_code
}

fn compile_new_content(output_path: &PathBuf, full_source: String) -> String {
  let current_output = fs::read_to_string(output_path)
    .expect(&format!("Error: failed to read from file: {:?}", output_path));
  if current_output != "" {
    let mut cartridge_iterator = current_output.split(HEADER_SEPARATOR);
    let header = cartridge_iterator.next()
      .expect("Error: failed to read the output file. The cartridge format seems to be incorrect.");
    let footer = cartridge_iterator.next()
      .expect("Error: failed to read the output file. The cartridge format seems to be incorrect.")
      .split(FOOTER_SEPARATOR)
      .last()
      .expect("Error: failed to read the output file. The cartridge format seems to be incorrect.");
    format!("{}{}{}{}{}", header, HEADER_SEPARATOR, full_source, FOOTER_SEPARATOR, footer)
  } else {
    format!("{}{}{}{}", PICO_HEADER, HEADER_SEPARATOR, full_source, FOOTER_SEPARATOR)
  }
}

fn recompile(sources: &Vec<PathBuf>, output_path: &PathBuf) {
  let full_source = concatenate_sources(sources);
  let new_content = compile_new_content(&output_path, full_source);
  let mut output_file = fs::OpenOptions::new().write(true).truncate(true).open(&output_path).unwrap();
  output_file.write_all(new_content.as_bytes()).expect("Error: failed to write to the output file.");
  output_file.sync_all().expect("Error: failed to write to the output file.");
}

fn main() {
  let opt = Opt::from_args();

  // Validating..
  let sources = check_input_files(&opt.input);
  let output_path = check_output_file(&opt.output);

  // Compiling..
  println!("Compiling {:?} into {:?}...", sources, output_path);
  recompile(&sources, &output_path);

  // Watch Mode
  if opt.watch {
    let sources = Mutex::new(sources);
    let is_compiling = Arc::new(Mutex::new(false));
    println!("Watching...");
    let mut hotwatch = Hotwatch::new()
      .expect("Error: Could not initialize watch mode. Please report this issue.");
    let is_compiling_mutex = Arc::clone(&is_compiling);
    hotwatch.watch(opt.input.clone(), move |event: Event| {
      match event {
        Event::Create(_) => {
          print!("Recompiling.. ");
          *is_compiling_mutex.lock().unwrap() = true;
          let mut sources = sources.lock().unwrap();
          *sources = check_input_files(&opt.input);
          recompile(&sources, &output_path);
          *is_compiling_mutex.lock().unwrap() = false;
          println!("Done!");
        },
        Event::NoticeWrite(_) => {
          print!("Recompiling.. ");
          *is_compiling_mutex.lock().unwrap() = true;
          recompile(&*sources.lock().unwrap(), &output_path);
          *is_compiling_mutex.lock().unwrap() = false;
          println!("Done!");
        },
        Event::NoticeRemove(_) => {
          print!("Recompiling.. ");
          *is_compiling_mutex.lock().unwrap() = true;
          let mut sources = sources.lock().unwrap();
          *sources = check_input_files(&opt.input);
          recompile(&sources, &output_path);
          *is_compiling_mutex.lock().unwrap() = false;
          println!("Done!");
        },
        Event::Rename(_, _) => {
          print!("Recompiling.. ");
          *is_compiling_mutex.lock().unwrap() = true;
          let mut sources = sources.lock().unwrap();
          *sources = check_input_files(&opt.input);
          recompile(&*sources, &output_path);
          *is_compiling_mutex.lock().unwrap() = false;
          println!("Done!");
        },
        _ => ()
      }
    }).expect("Failed to watch the files.");

    // Handle SIGINT & SIGTERM
    ctrlc::set_handler(move || {
      print!("Waiting for any builds to finish.. ");
      loop {
        if *is_compiling.lock().unwrap() {
          thread::sleep(time::Duration::from_millis(10));
        } else {
          println!("Done.");
          process::exit(0);
        }
      }
    }).unwrap();

    loop {
      thread::sleep(time::Duration::from_millis(1000));
    }
  }
}
