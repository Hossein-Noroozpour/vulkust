extern crate clap;
extern crate regex;

use std::fs::File;
use std::io::Read;

fn main() {
    let matches = clap::App::new("My Super Program")
        .version("1.0")
        .author("Hossein Noroozpour <hossein.noroozpour@gmail.com>")
        .about("Does doc cleaning stuff")
        .arg(
            clap::Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("FILE")
                .help("Sets the file that needs cleaning.")
                .takes_value(true),
        )
        .get_matches();
    let s = {
        let mut s = String::new();
        let mut f = File::open(matches.value_of("file").unwrap()).unwrap();
        f.read_to_string(&mut s).unwrap();
        s
    };
    let re = regex::Regex::new(r"[#][\[][d][o][c][ ][=][^\]]+[\]]\n").unwrap();
    let s = re.replace_all(&s, "");
    println!("{}", s);
}
