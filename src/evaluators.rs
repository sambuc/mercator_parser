use mercator_db::SpaceObject;

use super::expressions::*;
use super::symbols::*;

impl Evaluator for Predicate {
    fn eval(&self, object: &SpaceObject) -> bool {
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
