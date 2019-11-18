use std::collections::{HashMap, HashSet};

use mercator_db::space;
use mercator_db::Core;
use mercator_db::CoreQueryParameters;
use mercator_db::Properties;

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

fn complement_helper<'c>(
    core: &'c Core,
    parameters: &CoreQueryParameters<'c>,
    space_id: &str,
    inside: Vec<(&'c String, Vec<(space::Position, &'c Properties)>)>,
) -> mercator_db::ResultSet<'c> {
    let (low, high) = parameters.db.space(space_id)?.bounding_box();
    match core.get_by_shape(parameters, &space::Shape::BoundingBox(low, high), space_id) {
        e @ Err(_) => e,
        Ok(points) => {
            let hashmap = inside.into_iter().collect::<HashMap<_, _>>();

            Ok(points
                .into_iter()
                .filter_map(|(space, v)| match hashmap.get(space) {
                    None => None,
                    Some(list) => {
                        Some((space, v.into_iter().filter(|t| !list.contains(t)).collect()))
                    }
                })
                .collect::<Vec<_>>())
        }
    }
}

fn view_port<'c>(
    core_id: &str,
    parameters: &CoreQueryParameters<'c>,
    bag: &Bag,
) -> mercator_db::ResultSet<'c> {
    if let Some((low, high)) = parameters.view_port {
        let vp = Bag::Inside(Shape::HyperRectangle(
            bag.space().clone(),
            vec![low.into(), high.into()],
        ));
        intersection(core_id, parameters, &vp, bag)
    } else {
        bag.execute(core_id, parameters)
    }
}

fn distinct<'c>(
    core_id: &str,
    parameters: &CoreQueryParameters<'c>,
    bag: &Bag,
) -> mercator_db::ResultSet<'c> {
    match bag.execute(core_id, parameters) {
        e @ Err(_) => e,
        Ok(mut v) => {
            let set: HashSet<_> = v.drain(..).collect(); // dedup
            v.extend(set.into_iter());

            Ok(v)
        }
    }
}
fn filter_helper<'c>(
    predicate: &Predicate,
    bag: &Bag,
    core_id: &str,
    parameters: &CoreQueryParameters<'c>,
) -> mercator_db::ResultSet<'c> {
    match bag.execute(core_id, parameters) {
        e @ Err(_) => e,
        Ok(results) => Ok(results
            .into_iter()
            .filter_map(|(space, positions)| {
                let filtered = positions
                    .into_iter()
                    .filter(|(position, properties)| predicate.eval((space, position, properties)))
                    .collect::<Vec<_>>();
                if filtered.is_empty() {
                    None
                } else {
                    Some((space, filtered))
                }
            })
            .collect::<Vec<_>>()),
    }
}

fn filter<'c>(
    core_id: &str,
    parameters: &CoreQueryParameters<'c>,
    predicate: &Option<Predicate>,
    bag: &Option<Box<Bag>>,
) -> mercator_db::ResultSet<'c> {
    match predicate {
        None => {
            if let Some(bag) = bag {
                bag.execute(core_id, parameters)
            } else {
                Err("Filter without predicate nor data set.".to_string())
            }
        }
        Some(predicate) => match bag {
            None => {
                let (low, high) = space::Space::universe().bounding_box();
                let low: Vec<_> = low.into();
                let high: Vec<_> = high.into();
                let shape = Shape::HyperRectangle(
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
                filter_helper(predicate, &Bag::Inside(shape), core_id, parameters)
            }
            Some(bag) => filter_helper(predicate, bag.as_ref(), core_id, parameters),
        },
    }
}

fn complement<'c>(
    core_id: &str,
    parameters: &CoreQueryParameters<'c>,
    core: &'c Core,
    bag: &Bag,
) -> mercator_db::ResultSet<'c> {
    match bag.execute(core_id, parameters) {
        // FIXME: The complement of a set is computed within its definition space.
        e @ Err(_) => e,
        Ok(inside) => complement_helper(
            core,
            parameters,
            mercator_db::space::Space::universe().name(),
            inside,
        ),
    }
}

fn intersection<'c>(
    core_id: &str,
    parameters: &CoreQueryParameters<'c>,
    rh: &Bag,
    lh: &Bag,
) -> mercator_db::ResultSet<'c> {
    let l = lh.execute(core_id, parameters);
    if let Ok(l) = l {
        let r = rh.execute(core_id, parameters);
        if let Ok(r) = r {
            let mut v = vec![];

            if rh.predict(parameters.db) < lh.predict(parameters.db) {
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

fn union<'c>(
    core_id: &str,
    parameters: &CoreQueryParameters<'c>,
    rh: &Bag,
    lh: &Bag,
) -> mercator_db::ResultSet<'c> {
    let l = lh.execute(core_id, parameters);
    if let Ok(mut l) = l {
        let r = rh.execute(core_id, parameters);
        if let Ok(mut r) = r {
            if rh.predict(parameters.db) < lh.predict(parameters.db) {
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

fn bag<'c>(
    core_id: &str,
    parameters: &CoreQueryParameters<'c>,
    bags: &[Bag],
) -> mercator_db::ResultSet<'c> {
    let mut v = vec![];
    for bag in bags {
        let b = bag.execute(core_id, parameters);
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

fn inside<'c>(
    parameters: &CoreQueryParameters<'c>,
    core: &'c Core,
    shape: &Shape,
) -> mercator_db::ResultSet<'c> {
    let db = parameters.db;
    let param = match shape {
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
        Shape::Label(_, id) => {
            // Not a real shape, so short circuit and return.
            return core.get_by_label(parameters, id);
        }
        Shape::Nifti(_space_id) => Err("Inside-Nifti: not yet implemented".to_string()),
    };

    match param {
        Ok((space_id, shape)) => core.get_by_shape(parameters, &shape, space_id),
        Err(e) => Err(e),
    }
}

fn outside<'c>(
    parameters: &CoreQueryParameters<'c>,
    core: &'c Core,
    shape: &Shape,
) -> mercator_db::ResultSet<'c> {
    match shape {
        Shape::Point(space_id, position) => {
            let position: Vec<f64> = position.into();
            match core.get_by_positions(parameters, &[position.into()], space_id) {
                e @ Err(_) => e,
                Ok(inside) => complement_helper(core, parameters, space_id, inside),
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

            match core.get_by_shape(parameters, &space::Shape::BoundingBox(low, high), space_id) {
                e @ Err(_) => e,
                Ok(inside) => complement_helper(core, parameters, space_id, inside),
            }
        }
        Shape::HyperSphere(space_id, center, radius) => {
            // Smallest decrement possible, to exclude the surface
            let mut radius: f64 = radius.into();
            radius -= std::f64::EPSILON;
            let center: space::Position = center.into();

            match core.get_by_shape(
                parameters,
                &space::Shape::HyperSphere(center, radius.into()),
                space_id,
            ) {
                e @ Err(_) => e,
                Ok(inside) => complement_helper(core, parameters, space_id, inside),
            }
        }
        Shape::Label(_, _) => Err("Label: not yet implemented".to_string()),
        Shape::Nifti(_space_id) => Err("Outside-nifti: not yet implemented".to_string()),
    }
}

impl<'e> Executor<'e> for Projection {
    type ResultSet = mercator_db::ResultSet<'e>;

    fn execute<'f: 'e>(
        &self,
        core_id: &str,
        parameters: &CoreQueryParameters<'f>,
    ) -> Self::ResultSet {
        match self {
            Projection::Nifti(_, _, _bag) => Err("Proj-Nifti: not yet implemented".to_string()),
            Projection::JSON(_, _format, bag) => {
                bag.execute(core_id, parameters)
                // FIXME: Add projections here
            }
        }
    }
}

impl<'e> Executor<'e> for Bag {
    type ResultSet = mercator_db::ResultSet<'e>;

    fn execute<'f: 'e>(
        &self,
        core_id: &str,
        parameters: &CoreQueryParameters<'f>,
    ) -> Self::ResultSet {
        let core = parameters.db.core(core_id)?;

        match self {
            Bag::ViewPort(bag) => view_port(core_id, parameters, bag),
            Bag::Distinct(bag) => distinct(core_id, parameters, bag),
            Bag::Filter(predicate, bag) => filter(core_id, parameters, predicate, bag),
            Bag::Complement(bag) => complement(core_id, parameters, core, bag),
            Bag::Intersection(lh, rh) => intersection(core_id, parameters, rh, lh),
            Bag::Union(lh, rh) => union(core_id, parameters, rh, lh),
            Bag::Bag(list) => bag(core_id, parameters, list),
            Bag::Inside(shape) => inside(parameters, core, shape),
            Bag::Outside(shape) => {
                //FIXME: This is currently computed as the complement of the values within the shape, except its surface.
                //       Should this be instead a list of positions within the shape?
                //FIXME: Should we use the Shape's Space to get the maximum bounds or the output Space requested?
                outside(parameters, core, shape)
            }
        }
    }
}
