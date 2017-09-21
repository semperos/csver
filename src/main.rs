extern crate clap;
extern crate csver;

use clap::{App, Arg};
use std::io::{self, Read};
use std::process;

fn main() {
    let matches = App::new("csver")
        .version("0.1.0")
        .author("Daniel Gregoire <daniel.l.gregoire@gmail.com>")
        .about("JSON array over STDIN --> CSV over STDOUT")
        .arg(
            Arg::with_name("delimiter")
                .value_name("DELIMITER")
                .help("CSV delimiter to use between fields in each entry.")
                .default_value("comma")
                .long("delimiter")
                .short("d")
                .possible_values(&["comma", "tab"])
                .takes_value(true),
        )
        .get_matches();

    let delimiter_arg = matches.value_of("delimiter").unwrap().to_lowercase();
    let delimiter = match delimiter_arg.as_ref() {
        "comma" => b",",
        "tab" => b"\t",
        _ => {
            eprintln!("Unsupported CSV delimiter. Only 'comma' or 'tab' allowed.");
            process::exit(1);
        }
    };
    println!(
        "Delimiter arg was {} and final is {:?}",
        delimiter_arg,
        delimiter
    );

    let stdin = io::stdin();
    let mut buffer = String::new();
    stdin.lock().read_to_string(&mut buffer).expect(
        "Couldn't read from STDIN",
    );
    // TODO Figure out case when nothing comes in over stdin. Tools like grep just hang.
    if buffer.is_empty() {
        println!("{}", matches.usage());
        process::exit(1);
    } else if let Err(e) = csver::json_to_csv(&buffer, delimiter[0]) {
        eprintln!("Error converting JSON array to CSV: {:?}", e);
        process::exit(1);
    }
}
