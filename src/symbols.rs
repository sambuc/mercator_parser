use std::cmp::Ordering;

use mercator_db::space;
use mercator_db::Properties;

pub use super::types::*;

/**********************************************************************/
/* FORMATTING DATA                                                    */
/**********************************************************************/
#[derive(Clone, Debug)]
pub enum Projection {
    Nifti(String, LiteralSelector, Bag),
    JSON(String, JsonValue, Bag),
}

impl Projection {
    pub fn space(&self) -> &String {
        match self {
            Projection::Nifti(space, _, _) => &space,
            Projection::JSON(space, _, _) => &space,
        }
    }
}

// JSON FORMAT
#[derive(Clone, Debug)]
pub enum JsonValue {
    String(String),
    JsonNumber(LiteralNumber),
    Bool(bool),
    Null,
    Object(Vec<(String, JsonValue)>),
    Array(Vec<JsonValue>),
    Selector(LiteralSelector),
    Aggregation(Aggregation),
}

#[derive(Clone, Debug)]
pub enum Aggregation {
    Count(bool, LiteralSelector),
    Sum(LiteralSelector),
    Min(LiteralSelector),
    Max(LiteralSelector),
}

// NIFTI
#[derive(Clone, Debug)]
struct Transform {
    reference: String,
    offset: Vec<LiteralNumber>,
    rotation: Vec<Vec<LiteralNumber>>,
}

/**********************************************************************/
/* SELECTING / FILTERING DATA                                         */
/**********************************************************************/
#[derive(Clone, Debug)]
pub enum Bag {
    // This is an implicit operator, inserted by the parser. Never to be used directly.
    ViewPort(Box<Bag>),
    // Bags
    Distinct(Box<Bag>),
    Filter(Option<Predicate>, Box<Bag>),
    Complement(Box<Bag>),
    Intersection(Box<Bag>, Box<Bag>),
    Union(Box<Bag>, Box<Bag>),
    Bag(Vec<Bag>),
    Inside(Shape),
    Outside(Shape),
    //FIXME: ADD A SHAPE VARIANT WHICH JUST RETURNS ALL THE POSITIONS OF THAT SHAPE
    //Shape(Shape),
}

impl Bag {
    pub fn space(&self) -> &String {
        match self {
            Bag::ViewPort(bag) => bag.space(),
            Bag::Distinct(bag) => bag.space(),
            Bag::Filter(_, bag) => bag.space(),
            Bag::Complement(bag) => bag.space(),
            Bag::Intersection(lh, _) => {
                // We are assuming lh and rh are in the same space.
                // Checked as part of the validation.
                lh.space()
            }
            Bag::Union(lh, _) => {
                // We are assuming lh and rh are in the same space.
                // Checked as part of the validation.
                lh.space()
            }
            Bag::Bag(_) => {
                // Bags can be defined in different spaces, thus the output is
                // always in the universe space.
                space::Space::universe().name()
            }
            Bag::Inside(shape) => shape.space(),
            Bag::Outside(shape) => shape.space(),
        }
    }
}
/**********************************************************************/
/* BAG OPERATORS                                                      */
/**********************************************************************/
#[derive(Clone, Debug)]
pub enum Predicate {
    Less(Position, LiteralPosition),
    Greater(Position, LiteralPosition),
    Equal(Position, LiteralPosition),
    Not(Box<Predicate>),
    And(Box<Predicate>, Box<Predicate>),
    Or(Box<Predicate>, Box<Predicate>),
}

/**********************************************************************/
/* SPATIAL OPERATORS                                                  */
/**********************************************************************/

/**********************************************************************/
/* SHAPES                                                             */
/**********************************************************************/
#[derive(Clone, Debug)]
pub enum Shape {
    Point(String, LiteralPosition),
    HyperRectangle(String, Vec<LiteralPosition>),
    HyperSphere(String, LiteralPosition, LiteralNumber),
    Nifti(String),
}

impl Shape {
    pub fn space(&self) -> &String {
        match self {
            Shape::Point(space, _) => space,
            Shape::HyperRectangle(space, _) => space,
            Shape::HyperSphere(space, _, _) => space,
            Shape::Nifti(space) => space,
        }
    }

    pub fn volume(&self) -> f64 {
        match self {
            Shape::Point(_, _) => std::f64::EPSILON, // The smallest non-zero volume possible
            Shape::HyperRectangle(_space, pos) => {
                //TODO: At this time, only aligned to the axes, defined by two points, hyperrectangles are supported.
                assert_eq!(pos.len(), 2);

                // We assume the first position is the low point, the second is
                // the high point, this being true for each dimension. As we add
                // an even number of points per extra dimension, we assume the
                // last group is the high end, and the last position is the
                // highest point.
                let mut volume = 1.0;
                let LiteralPosition(low) = &pos[0];
                let LiteralPosition(high) = &pos[pos.len() - 1];

                // For each dimension, multiply by the length in that dimension
                for i in 0..low.len() {
                    let l = match low[i] {
                        LiteralNumber::Int(x) => x as f64,
                        LiteralNumber::Float(x) => x,
                    };

                    let h = match high[i] {
                        LiteralNumber::Int(x) => x as f64,
                        LiteralNumber::Float(x) => x,
                    };

                    let length = if h > l { h - l } else { l - h };
                    volume *= length;
                }

                volume
            }
            Shape::HyperSphere(_space, pos, radius) => {
                // Formula from https://en.wikipedia.org/wiki/N-sphere#/media/File:N_SpheresVolumeAndSurfaceArea.png
                let LiteralPosition(position) = pos;
                let k = position.len(); // Number of dimensions.

                let radius = match *radius {
                    LiteralNumber::Int(x) => x as f64,
                    LiteralNumber::Float(x) => x,
                };

                let pi = std::f64::consts::PI;
                let factor = 2.0 * pi;

                // Set starting values for the coefficient
                let mut a = 2.0;
                let mut i = if (k % 2) == 0 {
                    a = pi;
                    2
                } else {
                    1
                };

                while i < k {
                    i += 2;
                    a *= factor;
                    a /= i as f64;
                }

                a * radius.powi(i as i32)
            }
            Shape::Nifti(_) => unimplemented!(),
        }
    }
}

/**********************************************************************/
/* POSITIONS                                                          */
/**********************************************************************/
#[derive(Clone, Debug)]
pub enum Position {
    StrCmp(LiteralSelector, String),
    Selector(LiteralSelector),
    Literal(LiteralPosition),
}

impl Position {
    pub fn value<'e>(
        &self,
        object: (&'e String, &'e space::Position, &'e Properties),
    ) -> LiteralPosition {
        match self {
            Position::Literal(literal) => literal.clone(),
            Position::Selector(selector) => selector.position(object),
            Position::StrCmp(selector, literal) => {
                let x = match (selector.str(object)).cmp(literal) {
                    Ordering::Equal => 0,
                    Ordering::Greater => 1,
                    Ordering::Less => -1,
                };
                LiteralPosition(vec![LiteralNumber::Int(x)])
            }
        }
    }
}

/**********************************************************************/
/* Literals / TOKENS                                                  */
/**********************************************************************/

#[derive(Clone, Debug)]
pub struct Field(pub String, pub Option<usize>);

#[derive(Clone, Debug)]
pub enum LiteralNumber {
    Int(i64),
    Float(f64),
}

impl From<&LiteralNumber> for Vec<f64> {
    fn from(l: &LiteralNumber) -> Self {
        let r = match l {
            LiteralNumber::Int(x) => (*x) as f64,
            LiteralNumber::Float(x) => *x,
        };

        vec![r]
    }
}

impl PartialEq for LiteralNumber {
    fn eq(&self, other: &LiteralNumber) -> bool {
        match self {
            LiteralNumber::Int(l) => match other {
                LiteralNumber::Int(r) => l == r,
                LiteralNumber::Float(_) => false,
            },
            LiteralNumber::Float(l) => match other {
                LiteralNumber::Int(r) => l == &(*r as f64),
                LiteralNumber::Float(r) => l == r,
            },
        }
    }
}

#[derive(Clone, Debug)]
pub struct LiteralPosition(pub Vec<LiteralNumber>);

impl LiteralPosition {
    pub fn get_type(&self) -> LiteralTypes {
        let Self(v) = self;
        let mut t = Vec::new();

        for n in v {
            t.push(match n {
                LiteralNumber::Int(_) => LiteralTypes::Int,
                LiteralNumber::Float(_) => LiteralTypes::Float,
            });
        }

        LiteralTypes::Vector(t)
    }

    pub fn length(&self) -> f64 {
        let LiteralPosition(v) = self;
        let mut a = 0.0;

        for x in v {
            let x = match x {
                LiteralNumber::Int(x) => (*x) as f64,
                LiteralNumber::Float(x) => *x,
            };

            a += x * x;
        }

        a
    }

    pub fn dimensions(&self) -> usize {
        self.0.len()
    }
}

impl From<&LiteralNumber> for f64 {
    fn from(l: &LiteralNumber) -> Self {
        match l {
            LiteralNumber::Int(x) => (*x) as f64,
            LiteralNumber::Float(x) => *x,
        }
    }
}

impl From<&LiteralPosition> for Vec<f64> {
    fn from(l: &LiteralPosition) -> Self {
        let LiteralPosition(v) = l;
        let mut r = Vec::with_capacity(v.len());

        for x in v {
            let x = match x {
                LiteralNumber::Int(x) => (*x) as f64,
                LiteralNumber::Float(x) => *x,
            };
            r.push(x);
        }

        r
    }
}

impl From<&Vec<f64>> for LiteralPosition {
    fn from(v: &Vec<f64>) -> Self {
        let mut lv = Vec::with_capacity(v.len());
        for value in v {
            lv.push(LiteralNumber::Float(*value));
        }

        LiteralPosition(lv)
    }
}
impl From<&space::Position> for LiteralPosition {
    fn from(position: &space::Position) -> Self {
        let lv: Vec<f64> = position.into();
        (&lv).into()
    }
}

impl PartialOrd for LiteralPosition {
    fn partial_cmp(&self, other: &LiteralPosition) -> Option<Ordering> {
        let LiteralPosition(lh) = self;
        let LiteralPosition(rh) = other;

        if lh.len() != rh.len() {
            None
        } else {
            // Order is defined by the geometric length of the vector between the Origin and the point.
            let l = self.length();
            let r = other.length();

            l.partial_cmp(&r)
        }
    }
}

impl PartialEq for LiteralPosition {
    fn eq(&self, other: &LiteralPosition) -> bool {
        let LiteralPosition(lh) = self;
        let LiteralPosition(rh) = other;

        if lh.len() == rh.len() {
            for i in 0..lh.len() {
                if lh[i] != rh[i] {
                    return false;
                }
            }

            true
        } else {
            false
        }
    }
}

#[derive(Clone, Debug)]
pub struct LiteralSelector(pub Vec<Field>);

impl LiteralSelector {
    pub fn get_type(&self) -> LiteralTypes {
        // FIXME: Pretend for now that everything is a number, needs to be actually looked up in data model.
        LiteralTypes::Int
    }

    // FIXME: THIS IS SOOO WRONG
    pub fn position<'e>(
        &self,
        object: (&'e String, &'e space::Position, &'e Properties),
    ) -> LiteralPosition {
        println!("LiteralSelector.position(): {:?}", self);
        object.1.into()
    }

    // FIXME: THIS IS SOOO WRONG
    pub fn str<'e>(&self, object: (&'e String, &'e space::Position, &'e Properties)) -> &'e str {
        let LiteralSelector(v) = self;
        let last = v.last();
        if let Some(Field(name, _)) = last {
            if name == "id" {
                return object.2.id();
            } else if name == "type" {
                return object.2.type_name();
            } else if name == "reference_space" {
                return object.0;
            }
        }

        println!("LiteralSelector.str(): {:?}", self);
        unimplemented!();
    }
}

// The logic was getting a bit too complex to be embedded directly into the
// grammar definition.
pub fn get_filter(p: Predicate, b: Option<Bag>) -> Bag {
    match b {
        Some(b) => Bag::Filter(Some(p), Box::new(b)),
        None => {
            let (low, high) = space::Space::universe().bounding_box();
            let low: Vec<_> = low.into();
            let high: Vec<_> = high.into();
            let bb = Shape::HyperRectangle(
                space::Space::universe().name().clone(),
                vec![
                    LiteralPosition(
                        low.into_iter()
                            .map(LiteralNumber::Float)
                            .collect::<Vec<_>>(),
                    ),
                    LiteralPosition(
                        high.into_iter()
                            .map(LiteralNumber::Float)
                            .collect::<Vec<_>>(),
                    ),
                ],
            );

            Bag::Filter(Some(p), Box::new(Bag::Inside(bb)))
        }
    }
}

//FIXME: HACK FOR NOW; NEED PROPER TYPE CHECKING!
pub fn get_type() -> LiteralTypes {
    LiteralTypes::Vector(vec![
        LiteralTypes::Float,
        LiteralTypes::Float,
        LiteralTypes::Float,
    ])
}
