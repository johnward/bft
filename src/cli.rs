extern crate clap;
use clap::{App, Arg};

pub fn get_filename_and_cells() -> (String, usize) {
    let matches = App::new("My Super Program")
        .version("1.0")
        .author("John Ward <john@johnward.net>")
        .about("Does awesome things")
        .arg(
            Arg::with_name("cells")
                .short("c")
                .long("cells")
                .value_name("CELLS")
                .help("Sets the number of cells")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("PROGRAM")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .get_matches();

    let mut cell_size = 30000;
    let cells = matches.value_of("cells").unwrap();

    if cells.parse::<usize>().is_ok() && cells.parse::<usize>().unwrap() > cell_size {
        cell_size = cells.parse::<usize>().unwrap();
    }

    let program_name = String::from(matches.value_of("PROGRAM").unwrap_or("default.conf"));

    (program_name, cell_size)
}
