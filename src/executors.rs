use std::collections::HashSet;

use mercator_db::space;
use mercator_db::Core;
use mercator_db::DataBase;
use mercator_db::SpaceObject;

use super::expressions::*;
use super::symbols::*;

impl From<&LiteralPosition> for space::Position {
    fn from(literal: &LiteralPosition) -> Self {
        let v: Vec<f64> = literal.into();
        v.into()
    }
}

impl From<&LiteralNumber> for space::Coordinate {
    fn from(literal: &LiteralNumber) -> Self {
        match literal {
            LiteralNumber::Float(f) => (*f).into(),
            LiteralNumber::Int(i) => (*i as u64).into(),
        }
    }
}

fn complement_helper(
    db: &DataBase,
    core: &Core,
    space_id: &str,
    inside: Vec<SpaceObject>,
    output_space: Option<&str>,
    threshold: f64,
) -> mercator_db::ResultSet {
    let (low, high) = db.space(space_id)?.bounding_box();
    match core.get_by_shape(
        db,
        &space::Shape::BoundingBox(low, high),
        space_id,
        output_space,
        threshold,
    ) {
        e @ Err(_) => e,
        Ok(points) => Ok(points
            .into_iter()
            .filter(|o| !inside.contains(&o))
            .collect::<Vec<_>>()),
    }
}

fn distinct(
    db: &DataBase,
    core_id: &str,
    bag: &Bag,
    output_space: Option<&str>,
    threshold_volume: Option<f64>,
) -> mercator_db::ResultSet {
    match bag.execute(db, core_id, output_space, threshold_volume) {
        e @ Err(_) => e,
        Ok(mut v) => {
            let set: HashSet<_> = v.drain(..).collect(); // dedup
            v.extend(set.into_iter());

            Ok(v)
        }
    }
}

fn filter(
    db: &DataBase,
    core_id: &str,
    predicate: &Option<Predicate>,
    bag: &Bag,
    output_space: Option<&str>,
    threshold_volume: Option<f64>,
) -> mercator_db::ResultSet {
    match predicate {
        None => bag.execute(db, core_id, output_space, threshold_volume),
        Some(predicate) => match bag.execute(db, core_id, output_space, threshold_volume) {
            e @ Err(_) => e,
            Ok(results) => Ok(results
                .into_iter()
                .filter(|o| predicate.eval(&o))
                .collect::<Vec<_>>()),
        },
    }
}

fn complement(
    db: &DataBase,
    core_id: &str,
    core: &Core,
    bag: &Bag,
    output_space: Option<&str>,
    threshold: f64,
    threshold_volume: Option<f64>,
) -> mercator_db::ResultSet {
    match bag.execute(db, core_id, output_space, threshold_volume) {
        // FIXME: The complement of a set is computed within its definition space.
        e @ Err(_) => e,
        Ok(inside) => complement_helper(
            db,
            core,
            mercator_db::space::Space::universe().name(),
            inside,
            output_space,
            threshold,
        ),
    }
}

fn intersection(
    db: &DataBase,
    core_id: &str,
    rh: &Bag,
    lh: &Bag,
    output_space: Option<&str>,
    threshold_volume: Option<f64>,
) -> mercator_db::ResultSet {
    let l = lh.execute(db, core_id, output_space, threshold_volume);
    if let Ok(l) = l {
        let r = rh.execute(db, core_id, output_space, threshold_volume);
        if let Ok(r) = r {
            let mut v = vec![];

            if rh.predict(db) < lh.predict(db) {
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

fn union(
    db: &DataBase,
    core_id: &str,
    rh: &Bag,
    lh: &Bag,
    output_space: Option<&str>,
    threshold_volume: Option<f64>,
) -> mercator_db::ResultSet {
    let l = lh.execute(db, core_id, output_space, threshold_volume);
    if let Ok(mut l) = l {
        let r = rh.execute(db, core_id, output_space, threshold_volume);
        if let Ok(mut r) = r {
            if rh.predict(db) < lh.predict(db) {
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

fn bag(
    db: &DataBase,
    core_id: &str,
    bags: &[Bag],
    output_space: Option<&str>,
    threshold_volume: Option<f64>,
) -> mercator_db::ResultSet {
    let mut v = vec![];
    for bag in bags {
        let b = bag.execute(db, core_id, output_space, threshold_volume);
        match b {
            e @ Err(_) => {
                return e;
            }
            Ok(mut b) => {
                v.append(&mut b);
            }
        }
    }

    Ok(v)
}

fn inside(
    db: &DataBase,
    core: &Core,
    shape: &Shape,
    output_space: Option<&str>,
    threshold: f64,
) -> mercator_db::ResultSet {
    let parameters = match shape {
        Shape::Point(space_id, position) => {
            let space = db.space(space_id)?;
            let position: Vec<f64> = position.into();
            let position = space.encode(&position)?;
            Ok((space_id, space::Shape::Point(position)))
        }
        Shape::HyperRectangle(space_id, bounding_box) => {
            if bounding_box.len() != 2 {
                Err("The number of position is different from 2, which is unsupported.".to_string())
            } else {
                let space = db.space(space_id)?;
                let low: Vec<f64> = (&bounding_box[0]).into();
                let high: Vec<f64> = (&bounding_box[1]).into();
                let low = space.encode(&low)?;
                let high = space.encode(&high)?;

                Ok((space_id, space::Shape::BoundingBox(low, high)))
            }
        }
        Shape::HyperSphere(space_id, position, radius) => {
            let space = db.space(space_id)?;
            let position: Vec<f64> = position.into();
            let position = space.encode(&position)?;
            let mut r = vec![];
            for _ in 0..position.dimensions() {
                r.push(radius.into());
            }
            let radius = space.encode(&r)?[0];

            //FIXME: RADIUS IS A LENGTH, HOW TO ENCODE IT INTO THE SPACE?
            Ok((space_id, space::Shape::HyperSphere(position, radius)))
        }
        Shape::Nifti(_space_id) => Err("Inside-Nifti: not yet implemented".to_string()),
    };

    match parameters {
        Ok((space_id, shape)) => core.get_by_shape(db, &shape, space_id, output_space, threshold),
        Err(e) => Err(e),
    }
}

fn outside(
    db: &DataBase,
    core: &Core,
    shape: &Shape,
    output_space: Option<&str>,
    threshold: f64,
) -> mercator_db::ResultSet {
    match shape {
        Shape::Point(space_id, position) => {
            let position: Vec<f64> = position.into();
            match core.get_by_positions(db, &[position.into()], space_id, output_space, threshold) {
                e @ Err(_) => e,
                Ok(inside) => {
                    complement_helper(db, core, space_id, inside, output_space, threshold)
                }
            }
        }
        Shape::HyperRectangle(space_id, bounding_box) => {
            // We need to adapt the bounding_box to ensure the
            // surface will not hit as part of the inside set, so we
            // compute the biggest bounding box contained within the
            // given box.

            // Smallest increment possible
            let mut increment = Vec::with_capacity(bounding_box[0].dimensions());
            for _ in 0..bounding_box[0].dimensions() {
                increment.push(std::f64::EPSILON);
            }

            // Add it to the lower bound
            let mut low: space::Position = (&bounding_box[0]).into();
            low += increment.clone().into();

            // Substract it from the upper bound
            let mut high: space::Position = (&bounding_box[1]).into();
            high -= increment.into();

            match core.get_by_shape(
                db,
                &space::Shape::BoundingBox(low, high),
                space_id,
                output_space,
                threshold,
            ) {
                e @ Err(_) => e,
                Ok(inside) => {
                    complement_helper(db, core, space_id, inside, output_space, threshold)
                }
            }
        }
        Shape::HyperSphere(space_id, center, radius) => {
            // Smallest decrement possible, to exclude the surface
            let mut radius: f64 = radius.into();
            radius -= std::f64::EPSILON;
            let center: space::Position = center.into();

            match core.get_by_shape(
                db,
                &space::Shape::HyperSphere(center, radius.into()),
                space_id,
                output_space,
                threshold,
            ) {
                e @ Err(_) => e,
                Ok(inside) => {
                    complement_helper(db, core, space_id, inside, output_space, threshold)
                }
            }
        }
        Shape::Nifti(_space_id) => Err("Outside-nifti: not yet implemented".to_string()),
    }
}

impl Executor for Projection {
    type ResultSet = mercator_db::ResultSet;

    fn execute(
        &self,
        db: &DataBase,
        core_id: &str,
        output_space: Option<&str>,
        threshold_volume: Option<f64>,
    ) -> Self::ResultSet {
        match self {
            Projection::Nifti(_, _, _bag) => Err("Proj-Nifti: not yet implemented".to_string()),
            Projection::JSON(_, _format, bag) => {
                bag.execute(db, core_id, output_space, threshold_volume)
                // FIXME: Add projections here
            }
        }
    }
}

impl Executor for Bag {
    type ResultSet = mercator_db::ResultSet;

    fn execute(
        &self,
        db: &DataBase,
        core_id: &str,
        output_space: Option<&str>,
        threshold_volume: Option<f64>,
    ) -> Self::ResultSet {
        let threshold = match threshold_volume {
            None => 0.0,
            Some(v) => v,
        };
        let core = db.core(core_id)?;

        match self {
            Bag::Distinct(bag) => distinct(db, core_id, bag, output_space, threshold_volume),
            Bag::Filter(predicate, bag) => {
                filter(db, core_id, predicate, bag, output_space, threshold_volume)
            }
            Bag::Complement(bag) => complement(
                db,
                core_id,
                core,
                bag,
                output_space,
                threshold,
                threshold_volume,
            ),
            Bag::Intersection(lh, rh) => {
                intersection(db, core_id, rh, lh, output_space, threshold_volume)
            }
            Bag::Union(lh, rh) => union(db, core_id, rh, lh, output_space, threshold_volume),
            Bag::Bag(list) => bag(db, core_id, list, output_space, threshold_volume),
            Bag::Inside(shape) => inside(db, core, shape, output_space, threshold),
            Bag::Outside(shape) => {
                //FIXME: This is currently computed as the complement of the values within the shape, except its surface.
                //       Should this be instead a list of positions within the shape?
                //FIXME: Should we use the Shape's Space to get the maximum bounds or the output Space requested?
                outside(db, core, shape, output_space, threshold)
            }
        }
    }
}
