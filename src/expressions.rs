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

    fn execute(
        &self,
        db: &DataBase,
        core_id: &str,
        output_space: Option<&str>,
        threshold_volume: Option<f64>,
    ) -> Self::ResultSet;
}

pub trait Evaluator {
    fn eval(&self, object: &SpaceObject) -> bool;
}
