use mercator_db::space;
use mercator_db::Properties;

use super::expressions::*;
use super::symbols::*;

impl<'e> Evaluator<'e> for Predicate {
    type Object = (&'e String, &'e space::Position, &'e Properties);

    fn eval(&self, object: Self::Object) -> bool {
        match self {
            Predicate::Not(predicate) => !predicate.eval(object),
            Predicate::And(lh, rh) => lh.eval(object) && rh.eval(object),
            Predicate::Or(lh, rh) => lh.eval(object) || rh.eval(object),
            Predicate::Less(selector, literal) => &selector.value(object) < literal,
            Predicate::Greater(selector, literal) => &selector.value(object) > literal,
            Predicate::Equal(selector, literal) => &selector.value(object) == literal,
        }
    }
}
