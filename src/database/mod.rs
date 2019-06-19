use crate::executors::ResultSet;
use crate::symbols::*;

//use ironsea_index::*;

pub fn get_all(_space_id: &String) -> ResultSet {
    //space::get_all(space_id)
    Err("not yet implemented".to_string())
}

pub fn get_by_bounding_box(_space_id: &String, _bounding_box: &Vec<LiteralPosition>) -> ResultSet {
    Err("not yet implemented".to_string())
}

pub fn get_by_position(_space_id: &String, _position: &LiteralPosition) -> ResultSet {
    Err("not yet implemented".to_string())
}

pub fn get(_point: &Object, _fields: &Vec<Field>) -> Result<LiteralPosition, String> {
    Err("not yet implemented".to_string())
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Object {
    id: u32,
}

impl Object {
    pub fn length(&self) -> f64 {
        0.0
    }

    pub fn eval(&self, _predicate: &Predicate) -> bool {
        false
    }
}

impl Position {
    pub fn eval(&self, point: &Object) -> LiteralPosition {
        match self {
            Position::StrCmpICase(_selector, _string) => LiteralPosition(vec![]), //TODO
            Position::StrCmp(_selector, _string) => LiteralPosition(vec![]),      //TODO
            Position::Selector(selector) => selector.eval(point),
            Position::Literal(position) => position.clone(),
        }
    }
}

impl LiteralSelector {
    pub fn eval(&self, point: &Object) -> LiteralPosition {
        let LiteralSelector(fields) = self;
        match get(point, fields) {
            Err(_) => LiteralPosition(vec![]),
            Ok(p @ LiteralPosition(_)) => p,
        }
    }
}
