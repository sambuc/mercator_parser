#[macro_use]
extern crate measure_time;

extern crate parser;

use parser::queries;

use std::io;

fn main() {
    // If RUST_LOG is unset, set it to INFO, otherwise keep it as-is.
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();

    //let parser = queries::FiltersParser::new();
    let parser = queries::QueryParser::new();

    loop {
        info!("Expression to parse (type `quit` to exit): ");

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

        let input = input.as_str();
        let mut out;
        {
            debug_time!("Parsing");
            out = parser.parse(input);
        }
        info!("Tree: \n{:?}", out);
    }
}
