extern crate clap;
use clap::{Arg,App, SubCommand};


fn main() {
    let app = App::new("My Pipe")
       .version("1.0")
       .about("Does great things & some pipe")
       .author("Djadji Traore")
       .args(
         Arg::with_name("in")
          .long("in")
          .takes_value(true)
          .help("In command")
          .required(true);
     )
     .args(
         Arg::with_name("out")
            .long("out")
            .takes_value(true)
            .help("out command")
            .required(true)
     )
    .get_matches();
}
