use std::collections::HashMap;

use crate::types::LiteralTypes;

//FIXME: Improve, as this should not be static, but coming from DB.
lazy_static! {
    static ref UNIVERSE: String = "Universe".to_string();
    static ref SPACES: HashMap<&'static str, Space> = {
        let mut m = HashMap::new();
        m.insert(
            "Universe",
            Space {
                space_type: LiteralTypes::Vector(vec![
                    LiteralTypes::Float,
                    LiteralTypes::Float,
                    LiteralTypes::Float,
                ]),
                bounding_box: vec![
                    vec![0, 0, 0],
                    vec![
                        std::u32::MAX as i64,
                        std::u32::MAX as i64,
                        std::u32::MAX as i64,
                    ],
                ],
            },
        );

        m
    };
}

struct Space {
    space_type: LiteralTypes,
    bounding_box: Vec<Vec<i64>>,
}

impl Space {
    pub fn max_volume(&self) -> f64 {
        let mut volume = 1.0;
        for max in &self.bounding_box[1] {
            volume *= *max as f64;
        }

        volume
    }
}

pub fn name() -> &'static String {
    lazy_static! {
        static ref UNIVERSE: String = "Universe".to_string();
    };

    &UNIVERSE
}

#[inline]
pub fn get_type(space_id: &String) -> &LiteralTypes {
    lazy_static! {
        static ref EMPTY_TYPE: LiteralTypes = LiteralTypes::Vector(Vec::new());
    };

    match SPACES.get(space_id.as_str()) {
        None => &EMPTY_TYPE,
        Some(space) => &space.space_type,
    }
}

#[inline]
pub fn bounding_box(space_id: &String) -> &Vec<Vec<i64>> {
    lazy_static! {
        static ref EMPTY_BOX: Vec<Vec<i64>> = Vec::new();
    };
    match SPACES.get(space_id.as_str()) {
        None => &EMPTY_BOX,
        Some(space) => &space.bounding_box,
    }
}

#[inline]
pub fn max_volume(space_id: &String) -> f64 {
    match SPACES.get(space_id.as_str()) {
        None => 0.0,
        Some(space) => space.max_volume(),
    }
}
