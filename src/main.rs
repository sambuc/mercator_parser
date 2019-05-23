extern crate parser;

use parser::queries;

use std::io;

fn main() {
    //let parser = queries::FiltersParser::new();
    let parser = queries::QueryParser::new();

    loop {
        println!("\n> Expression to parse (type `quit` to exit): ");

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(0) => break,    // Catch ^D
            Ok(1) => continue, // Catch \n
            Err(_) => continue,
            Ok(_) => (),
        }

        if input.trim().eq_ignore_ascii_case("quit") {
            break;
        }
        println!("\n> Tree: \n{:?}", parser.parse(input.as_str()));
    }
}
