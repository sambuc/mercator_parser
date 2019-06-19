#[macro_use]
extern crate measure_time;

extern crate parser;

use parser::QueryParser;
use parser::{Executor, Predictor, Validator};

use std::io;

fn main() {
    // If RUST_LOG is unset, set it to INFO, otherwise keep it as-is.
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();

    //let parser = queries::FiltersParser::new();
    let parser = QueryParser::new();

    loop {
        println!();
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
        {
            debug_time!("Interpretation");
            let mut parse;
            {
                trace_time!("Parsing");
                parse = parser.parse(input);
            }
            trace!("Tree: \n{:?}", parse);

            match parse {
                Ok(Some(t)) => {
                    let validate;
                    {
                        trace_time!("Type check");
                        validate = t.validate();
                    }
                    info!("Type: \n{:?}", validate);

                    if let Ok(_) = validate {
                        let predict;
                        {
                            trace_time!("Prediction");
                            predict = t.predict();
                        }
                        info!("Predict: \n{:?}", predict);

                        let execute;
                        {
                            trace_time!("Exectution");
                            execute = t.execute();
                        }
                        info!("Execution: \n{:?}", execute);
                    }
                }
                _ => (),
            }
        }
    }
}
