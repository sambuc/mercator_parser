use mercator_db::CoreQueryParameters;
use mercator_db::DataBase;
use mercator_db::SpaceObject;

pub trait Validator {
    type ValidationResult;

    fn validate(&self) -> Self::ValidationResult;
}

pub trait Predictor {
    fn predict(&self, db: &DataBase) -> Result<f64, String>;
}

pub trait Executor {
    type ResultSet;

    fn execute(&self, core_id: &str, parameters: &CoreQueryParameters) -> Self::ResultSet;
}

pub trait Evaluator {
    fn eval(&self, object: &SpaceObject) -> bool;
}
