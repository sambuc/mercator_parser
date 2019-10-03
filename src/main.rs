#[macro_use]
extern crate measure_time;

use std::io;
use std::process::exit;

use mercator_db::json::storage;
use mercator_db::CoreQueryParameters;
use mercator_db::DataBase;
use parser::Executor;
use parser::FiltersParser;
use parser::Predictor;
use parser::QueryParser;
use parser::Validator;

fn main() {
    // If RUST_LOG is unset, set it to INFO, otherwise keep it as-is.
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();

    let import;

    if std::env::var("MERCATOR_IMPORT_DATA").is_err() {
        std::env::set_var("MERCATOR_IMPORT_DATA", "test");
    }

    match std::env::var("MERCATOR_IMPORT_DATA") {
        Ok(val) => import = val,
        Err(val) => {
            error!("Could not fetch {} : `{}`", "MERCATOR_IMPORT_DATA", val);
            exit(1);
        }
    };

    // Convert to binary the JSON data:
    if true {
        info_time!("Converting to binary JSON data");
        storage::convert(&import);
    }

    // Build a Database Index:
    if true {
        info_time!("Building database index");
        storage::build(&import);
    }

    // Load a Database:
    let db;
    {
        info_time!("Loading database index");
        db = DataBase::load(import).unwrap();
    }

    let parameters = CoreQueryParameters {
        db: &db,
        output_space: None,
        threshold_volume: None,
        resolution: None,
    };
    let parser = QueryParser::new();
    let parser = FiltersParser::new();

    loop {
        println!();
        info!("Expression to parse (type `quit` to exit): ");

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => break,    // Catch ^D
            Ok(1) => continue, // Catch \n
            Err(_) => continue,
            Ok(_) => {
                if input.trim().eq_ignore_ascii_case("quit") {
                    break;
                }

                info_time!("Interpretation");
                let mut parse;
                {
                    info_time!("Parsing");
                    parse = parser.parse(&input);
                }

                if let Err(e) = &parse {
                    warn!("Parsing failed: \n{:?}", e);
                } else {
                    trace!("Tree: \n{:?}", parse);
                }

                // QueryParser
                //if let Ok(Some(t)) = parse {

                // FiltersParser
                if let Ok(t) = parse {
                    let validate;
                    {
                        info_time!("Type check");
                        validate = t.validate();
                    }
                    info!("Type: \n{:?}", validate);

                    if validate.is_ok() {
                        let predict;
                        {
                            info_time!("Prediction");
                            predict = t.predict(&db);
                        }
                        info!("Predict: \n{:?}", predict);

                        let execute;
                        {
                            info_time!("Execution");
                            execute = t.execute("test", &parameters);
                        }

                        if let Ok(r) = execute {
                            //let r = model::to_spatial_objects(&db, r);
                            info!("Execution: \n{:#?}", r);
                            info!("NB results: {:?}", r.len());
                        } else {
                            info!("Execution: \n{:?}", execute);
                        }
                    }
                }
            }
        }
    }
}
