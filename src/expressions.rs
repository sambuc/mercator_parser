use mercator_db::CoreQueryParameters;
use mercator_db::DataBase;

pub trait Validator {
    type ValidationResult;

    fn validate(&self) -> Self::ValidationResult;
}

pub trait Predictor {
    fn predict(&self, db: &DataBase) -> Result<f64, String>;
}

pub trait Executor<'e> {
    type ResultSet;

    fn execute(
        &'e self,
        core_id: &'e str,
        parameters: &'e CoreQueryParameters<'e>,
    ) -> Self::ResultSet;
}

pub trait Evaluator<'e> {
    type Object;

    fn eval(&self, object: Self::Object) -> bool;
}
