use std::collections::HashSet;

use super::database;
use super::expression::*;
use super::symbols::*;

pub type ResultSet = Result<Vec<database::Object>, String>;

impl Executor for Projection {
    type ResultSet = self::ResultSet;

    fn execute(&self) -> ResultSet {
        match self {
            Projection::Nifti(_, _, _bag) => Err("not yet implemented".to_string()),
            Projection::JSON(_, _format, bag) => bag.execute(), // FIXME: Add projections here
        }
    }
}

impl Executor for Bag {
    type ResultSet = self::ResultSet;

    fn execute(&self) -> ResultSet {
        fn get_bounding_box(
            position: &LiteralPosition,
            radius: &LiteralNumber,
        ) -> Result<Vec<LiteralPosition>, String> {
            let LiteralPosition(position) = position;
            let mut low = vec![];
            let mut high = vec![];
            match radius {
                LiteralNumber::Int(r) => {
                    for x in position {
                        match x {
                            LiteralNumber::Int(x) => {
                                low.push(LiteralNumber::Int(x - r));
                                high.push(LiteralNumber::Int(x + r));
                            }
                            LiteralNumber::Float(x) => {
                                low.push(LiteralNumber::Float(x - (*r as f64)));
                                high.push(LiteralNumber::Float(x + (*r as f64)));
                            }
                        };
                    }
                }
                LiteralNumber::Float(r) => {
                    for x in position {
                        match x {
                            LiteralNumber::Int(_) => {
                                return Err(format!("The radius provided is a floating point value, which is incompatible with integer coordinates components: radius {:?}, coordinates {:?}", radius, position));
                            }
                            LiteralNumber::Float(x) => {
                                low.push(LiteralNumber::Float(x - r));
                                high.push(LiteralNumber::Float(x + r));
                            }
                        };
                    }
                }
            }

            Ok(vec![LiteralPosition(low), LiteralPosition(high)])
        };

        match self {
            Bag::Distinct(bag) => match bag.execute() {
                e @ Err(_) => e,
                Ok(mut v) => {
                    let set: HashSet<_> = v.drain(..).collect(); // dedup
                    v.extend(set.into_iter());

                    Ok(v)
                }
            },
            Bag::Filter(predicate, bag) => match predicate {
                None => bag.execute(),
                Some(predicate) => match bag.execute() {
                    e @ Err(_) => e,
                    Ok(source) => {
                        let mut filtered = Vec::new();

                        for point in source {
                            if point.eval(predicate) {
                                filtered.push(point);
                            }
                        }

                        Ok(filtered)
                    }
                },
            },
            Bag::Complement(bag) => match bag.execute() {
                // The complement of a set is computed within its definition space.
                e @ Err(_) => e,
                Ok(inside) => {
                    let mut outside = Vec::new();
                    match database::get_all(bag.space()) {
                        e @ Err(_) => e,
                        Ok(points) => {
                            for point in points {
                                if !inside.contains(&point) {
                                    outside.push(point)
                                }
                            }

                            Ok(outside)
                        }
                    }
                }
            },
            Bag::Intersection(lh, rh) => {
                let l = lh.execute();
                if let Ok(l) = l {
                    let r = rh.execute();
                    if let Ok(r) = r {
                        let mut v = vec![];

                        if rh.predict() < lh.predict() {
                            for o in r {
                                if l.contains(&o) {
                                    v.push(o);
                                }
                            }
                        } else {
                            for o in l {
                                if r.contains(&o) {
                                    v.push(o);
                                }
                            }
                        }
                        Ok(v)
                    } else {
                        r
                    }
                } else {
                    l
                }
            }
            Bag::Union(lh, rh) => {
                let l = lh.execute();
                if let Ok(mut l) = l {
                    let r = rh.execute();
                    if let Ok(mut r) = r {
                        if rh.predict() < lh.predict() {
                            l.append(&mut r);
                            Ok(l)
                        } else {
                            r.append(&mut l);
                            Ok(r)
                        }
                    } else {
                        r
                    }
                } else {
                    l
                }
            }
            Bag::Bag(bags) => {
                let mut v = vec![];
                for bag in bags {
                    let b = bag.execute();
                    match b {
                        e @ Err(_) => {
                            return e;
                        }
                        Ok(mut b) => {
                            //TODO: SPACE CONVERSIONS IF NOT THE SAME SPACES?
                            v.append(&mut b);
                        }
                    }
                }

                Ok(v)
            }
            Bag::Inside(shape) => match shape {
                Shape::Point(space_id, position) => database::get_by_position(space_id, position),
                Shape::HyperRectangle(space_id, bounding_box) => {
                    database::get_by_bounding_box(space_id, bounding_box)
                }
                Shape::HyperSphere(space_id, position, radius) => {
                    let length = match radius {
                        LiteralNumber::Int(x) => *x as f64,
                        LiteralNumber::Float(x) => *x,
                    };

                    match get_bounding_box(position, radius) {
                        Err(e) => Err(e),
                        Ok(inside) => match database::get_by_bounding_box(space_id, &inside) {
                            e @ Err(_) => e,
                            Ok(source) => {
                                let mut filtered = vec![];

                                for point in source {
                                    // Include the surface of the sphere
                                    if point.length() <= length {
                                        filtered.push(point);
                                    }
                                }
                                Ok(filtered)
                            }
                        },
                    }
                }
                Shape::Nifti(_space_id) => Err("not yet implemented".to_string()),
            },
            Bag::Outside(shape) => {
                fn outside_set(space_id: &String, inside: Vec<database::Object>) -> ResultSet {
                    let mut outside = Vec::new();
                    match database::get_all(space_id) {
                        e @ Err(_) => e,
                        Ok(points) => {
                            for point in points {
                                if !inside.contains(&point) {
                                    outside.push(point)
                                }
                            }

                            Ok(outside)
                        }
                    }
                }

                match shape {
                    Shape::Point(space_id, position) => {
                        match database::get_by_position(space_id, position) {
                            e @ Err(_) => e,
                            Ok(inside) => outside_set(space_id, inside),
                        }
                    }
                    Shape::HyperRectangle(space_id, bounding_box) => {
                        // We need to adapt the bounding_box to ensure the
                        // surface will not hit as part of the inside set, so we
                        // compute the biggest bounding box contained within the
                        // given box.

                        // Smallest increment possible
                        let mut low: Vec<LiteralNumber> = vec![];
                        let LiteralPosition(coordinates) = &bounding_box[0];
                        for coordinate in coordinates {
                            match coordinate {
                                LiteralNumber::Int(x) => low.push(LiteralNumber::Int(x + 1)),
                                LiteralNumber::Float(x) => {
                                    low.push(LiteralNumber::Float(x + std::f64::EPSILON))
                                }
                            };
                        }
                        let low = LiteralPosition(low);

                        // Smallest decrement possible
                        let mut high: Vec<LiteralNumber> = vec![];
                        let LiteralPosition(coordinates) = &bounding_box[1];
                        for coordinate in coordinates {
                            match coordinate {
                                LiteralNumber::Int(x) => high.push(LiteralNumber::Int(x - 1)),
                                LiteralNumber::Float(x) => {
                                    high.push(LiteralNumber::Float(x - std::f64::EPSILON))
                                }
                            };
                        }
                        let high = LiteralPosition(high);

                        match database::get_by_bounding_box(space_id, &vec![low, high]) {
                            e @ Err(_) => e,
                            Ok(inside) => outside_set(space_id, inside),
                        }
                    }
                    Shape::HyperSphere(space_id, position, radius) => {
                        let length = match radius {
                            LiteralNumber::Int(x) => *x as f64,
                            LiteralNumber::Float(x) => *x,
                        };

                        match get_bounding_box(position, radius) {
                            Err(e) => Err(e),
                            Ok(inside) => match database::get_by_bounding_box(space_id, &inside) {
                                Err(e) => Err(e),
                                Ok(source) => {
                                    let mut filtered = vec![];

                                    for point in source {
                                        // Exclude the surface of the sphere, so
                                        // that it is included in the
                                        // complement.
                                        if point.length() < length {
                                            filtered.push(point);
                                        }
                                    }

                                    outside_set(space_id, filtered)
                                }
                            },
                        }
                    }
                    Shape::Nifti(_space_id) => Err("not yet implemented".to_string()),
                }
            }
        }
    }
}

impl Predicate {
    pub fn eval(&self, point: &database::Object) -> bool {
        match self {
            Predicate::Not(predicate) => !predicate.eval(point),
            Predicate::And(lh, rh) => lh.eval(point) && rh.eval(point),
            Predicate::Or(lh, rh) => lh.eval(point) || rh.eval(point),
            // I don't know how to evaluate these at this point, so let the DB object take care of that.
            //            Predicate::Less(selector, literal) => &selector.eval(point) < literal,
            //            Predicate::Greater(selector, literal) => &selector.eval(point) > literal,
            //            Predicate::Equal(selector, literal) => &selector.eval(point) == literal,
            // Redirect to the DB Objet the evaluation of the remaining predicate operators
            predicate => point.eval(predicate),
        }
    }
}
