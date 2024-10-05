extern crate hidapi;
mod colors;
mod hid;

use clap::Parser;
use colors::{apply_color};
use hid::get_device;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
  #[arg(short, long)]
  c: Option<String>,
}

fn main() {
  let args = Args::parse();
  if let Some(device) = get_device(1008, 3214) {
    let color = args.c.unwrap_or_default();
    if color.len() > 0 {
        apply_color(&device, color.as_str());
    }
  }
}