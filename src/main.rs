mod config_handler;
use std::{
  io,
  fs,
  env,
};


fn main() {
  let args:Vec<String> = env::args().collect();
  let configstr = fs::read_to_string(args[1].clone()).unwrap();
  println!("{:#?}", config_handler::parse(configstr));
}
