pub trait Validator {
    type ValidationResult;

    fn validate(&self) -> Self::ValidationResult;
}

pub trait Predictor {
    fn predict(&self) -> f64;
}

pub trait Executor {
    type ResultSet;

    fn execute(&self) -> Self::ResultSet;
}
