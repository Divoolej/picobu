use structopt::StructOpt;
use std::path::{Path, PathBuf};

#[derive(Debug, StructOpt)]
struct Opt {
  #[structopt(short = "i", long = "input", parse(from_os_str), default_value = "src/")]
  input: PathBuf,
  #[structopt(parse(from_os_str))]
  output: Option<PathBuf>
}

fn main() {
  let opt = Opt::from_args();
}
