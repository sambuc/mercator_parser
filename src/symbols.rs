use std::cmp::Ordering;

use super::database::space;
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
    Distinct(Box<Bag>),
    Filter(Option<Predicate>, Box<Bag>),
    Complement(Box<Bag>),
    Intersection(Box<Bag>, Box<Bag>),
    Union(Box<Bag>, Box<Bag>),
    Bag(Vec<Bag>),
    Inside(Shape),
    Outside(Shape),
}

impl Bag {
    pub fn space(&self) -> &String {
        match self {
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
                space::name()
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
            Shape::Point(_, _) => 1.0, // This is the smallest non-zero volume possible //TODO DOUBLE CHECK IT IS TRUE
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
            Shape::HyperSphere(_space, pos, r) => {
                // Formula from https://en.wikipedia.org/wiki/N-sphere#/media/File:N_SpheresVolumeAndSurfaceArea.png
                let LiteralPosition(p) = pos;
                let k = p.len(); // Number of dimensions.

                let r = match *r {
                    LiteralNumber::Int(x) => x as f64,
                    LiteralNumber::Float(x) => x,
                };

                let pi = std::f64::consts::PI;
                let factor = 2.0 * pi;

                // Set starting values for the coefficient
                let mut a = 2.0;
                let mut i = 1;

                if (k % 2) == 0 {
                    a = pi;
                    i = 2;
                }

                while i < k {
                    i += 2;
                    a *= factor;
                    a /= i as f64;
                }

                a * r.powi(i as i32)
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
    StrCmpICase(LiteralSelector, String),
    StrCmp(LiteralSelector, String),
    Selector(LiteralSelector),
    Literal(LiteralPosition),
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
}
