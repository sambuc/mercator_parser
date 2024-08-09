use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use mercator_db::space;
use mercator_db::Core;
use mercator_db::CoreQueryParameters;
use mercator_db::IterObjects;
use mercator_db::IterObjectsBySpaces;

use super::expressions::*;
use super::symbols::*;

fn group_by_space<'s>(
    list: IterObjectsBySpaces<'s>,
) -> Box<dyn Iterator<Item = (&'s String, IterObjects<'s>)> + 's> {
    // Filter per Properties, in order to regroup by it, then build
    // a single SpatialObject per Properties.
    let mut hashmap = HashMap::new();
    for (space, objects) in list {
        hashmap.entry(space).or_insert_with(Vec::new).push(objects);
    }

    Box::new(hashmap.into_iter().map(|(space, objects)| {
        let objects: IterObjects = Box::new(objects.into_iter().flatten());
        (space, objects)
    }))
}

fn distinct_helper(list: IterObjectsBySpaces) -> IterObjectsBySpaces {
    // Make sure to collect all objects iterators per space, so that
    // each space appears only once.
    group_by_space(list)
        // We would lose some objects otherwise when creating the
        // HashMaps. Also this makes sure to keep the values are unique.
        .map(|(space, iter)| {
            let uniques: HashSet<_> = iter.collect();
            let uniques: IterObjects = Box::new(uniques.into_iter());
            (space, uniques)
        })
        .collect()
}

fn into_positions_hashset(
    objects_by_spaces: IterObjectsBySpaces,
) -> HashMap<&String, Rc<HashSet<space::Position>>> {
    // Make sure to collect all objects iterators per space, so that
    // each space appears only once.
    group_by_space(objects_by_spaces)
        // We would lose some objects otherwise when creating the HashSets.
        .map(|(space, iter)| {
            let hash_set: HashSet<_> = iter.map(|(position, _)| position).collect();
            (space, Rc::new(hash_set))
        })
        .collect::<HashMap<_, _>>()
}

// Strictly not inside nor on the surface.
// TODO: inside must contains the valid positions in all expected spaces
fn complement_helper<'h>(
    core: &'h Core,
    parameters: &'h CoreQueryParameters<'h>,
    space_id: &'h str,
    inside: IterObjectsBySpaces<'h>,
) -> mercator_db::ResultSet<'h> {
    let (low, high) = parameters.db.space(space_id)?.bounding_box();
    let inside = into_positions_hashset(inside);
    let points = core.get_by_shape(parameters, space::Shape::BoundingBox(low, high), space_id)?;

    let results = points
        .into_iter()
        .filter_map(move |(space, v)| match inside.get(space) {
            None => None, // Space not found, so no point might exist!
            Some(volume) => {
                let volume = volume.clone();
                let iter: IterObjects = Box::new(v.filter(move |a| !volume.contains(&a.0)));

                Some((space, iter))
            }
        })
        .collect();
    Ok(results)
}

// Intersection based only on spatial positions!
fn intersect_helper<'h>(
    smaller: IterObjectsBySpaces<'h>,
    bigger: IterObjectsBySpaces<'h>,
) -> IterObjectsBySpaces<'h> {
    let smaller = into_positions_hashset(smaller);

    bigger
        .into_iter()
        .filter_map(
            move |(space, bigger_object_iter)| match smaller.get(space) {
                None => None,
                Some(volume) => {
                    let volume = volume.clone();
                    let filtered: IterObjects =
                        Box::new(bigger_object_iter.filter(move |a| volume.contains(&a.0)));

                    Some((space, filtered))
                }
            },
        )
        .collect()
}

impl Bag {
    fn distinct<'b>(
        &'b self,
        core_id: &'b str,
        parameters: &'b CoreQueryParameters<'b>,
    ) -> mercator_db::ResultSet<'b> {
        let results = self.execute(core_id, parameters)?;

        Ok(distinct_helper(results))
    }

    fn complement<'b>(
        &'b self,
        core_id: &'b str,
        parameters: &'b CoreQueryParameters<'b>,
        core: &'b Core,
    ) -> mercator_db::ResultSet<'b> {
        let inside = self.execute(core_id, parameters)?;

        // FIXME: The complement of a set should be computed within its
        //        definition space. We don't know here so we use universe
        complement_helper(
            core,
            parameters,
            mercator_db::space::Space::universe().name(),
            inside,
        )
    }

    fn intersection<'b>(
        &'b self,
        core_id: &'b str,
        parameters: &'b CoreQueryParameters<'b>,
        rh: &'b Bag,
    ) -> mercator_db::ResultSet<'b> {
        let left = self.execute(core_id, parameters)?;
        let right = rh.execute(core_id, parameters)?;

        let v = if rh.predict(parameters.db) < self.predict(parameters.db) {
            intersect_helper(right, left)
        } else {
            intersect_helper(left, right)
        };

        Ok(v)
    }

    fn union<'b>(
        &'b self,
        core_id: &'b str,
        parameters: &'b CoreQueryParameters<'b>,
        rh: &'b Bag,
    ) -> mercator_db::ResultSet<'b> {
        let mut left = self.execute(core_id, parameters)?;
        let mut right = rh.execute(core_id, parameters)?;

        let union = if rh.predict(parameters.db) < self.predict(parameters.db) {
            left.append(&mut right);
            left
        } else {
            right.append(&mut left);
            right
        };

        Ok(union)
    }

    fn filter<'b>(
        &'b self,
        predicate: &'b Predicate,
        core_id: &'b str,
        parameters: &'b CoreQueryParameters<'b>,
    ) -> mercator_db::ResultSet<'b> {
        let results = self.execute(core_id, parameters)?;

        Ok(results
            .into_iter()
            .map(move |(space, positions)| {
                let positions = positions.collect::<Vec<_>>();
                (
                    space,
                    Box::new(positions.into_iter().filter(move |(position, properties)| {
                        predicate.eval((space, position, properties))
                    })) as IterObjects,
                )
            })
            .collect())
    }
}

impl Shape {
    fn inside<'s>(
        &'s self,
        parameters: &'s CoreQueryParameters<'s>,
        core: &'s Core,
    ) -> mercator_db::ResultSet<'s> {
        let db = parameters.db;
        let param = match self {
            Shape::Point(space_id, position) => {
                let space = db.space(space_id)?;
                let position: Vec<f64> = position.into();
                let position = space.encode(&position)?;
                Ok((space_id, space::Shape::Point(position)))
            }
            Shape::HyperRectangle(space_id, bounding_box) => {
                if bounding_box.len() != 2 {
                    //FIXME: Support arbitrary HyperRectangles
                    Err(
                        "The number of position is different from 2, which is unsupported."
                            .to_string(),
                    )
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

                // We have to provide a position with all the dimensions
                // for the encoding to work as expected.
                let mut r = vec![0f64; position.dimensions()];
                r[0] = radius.into();
                let radius = space.encode(&r)?[0];

                Ok((space_id, space::Shape::HyperSphere(position, radius)))
            }
            Shape::Label(_, id) => {
                // Not a real shape, so short circuit and return.
                return core.get_by_label(parameters, id);
            }
            Shape::Nifti(_space_id) => Err("Inside-Nifti: not yet implemented".to_string()),
        };

        match param {
            Ok((space_id, shape)) => core.get_by_shape(parameters, shape, space_id),
            Err(e) => Err(e),
        }
    }

    fn outside<'s>(
        &'s self,
        parameters: &'s CoreQueryParameters<'s>,
        core: &'s Core,
    ) -> mercator_db::ResultSet<'s> {
        let (space_id, inside) = match self {
            Shape::Point(space_id, position) => {
                let position: Vec<f64> = position.into();
                let positions = vec![position.into()];
                let inside = core.get_by_positions(parameters, positions, space_id)?;

                Ok((space_id, inside))
            }
            Shape::HyperRectangle(space_id, bounding_box) => {
                // We need to adapt the bounding_box to ensure the
                // surface will not hit as part of the inside set, so we
                // compute the biggest bounding box contained within the
                // given box.

                // Smallest increment possible
                let mut increment = Vec::with_capacity(bounding_box[0].dimensions());
                for _ in 0..bounding_box[0].dimensions() {
                    increment.push(f64::EPSILON);
                }

                // Add it to the lower bound
                let mut low: space::Position = (&bounding_box[0]).into();
                low += increment.clone().into();

                // Substract it from the upper bound
                let mut high: space::Position = (&bounding_box[1]).into();
                high -= increment.into();

                let inside =
                    core.get_by_shape(parameters, space::Shape::BoundingBox(low, high), space_id)?;

                Ok((space_id, inside))
            }
            Shape::HyperSphere(space_id, center, radius) => {
                // Smallest decrement possible, to exclude the surface
                let mut radius: f64 = radius.into();
                radius -= f64::EPSILON;
                let center: space::Position = center.into();

                let inside = core.get_by_shape(
                    parameters,
                    space::Shape::HyperSphere(center, radius.into()),
                    space_id,
                )?;

                Ok((space_id, inside))
            }
            Shape::Label(space_id, id) => {
                let inside = core.get_by_label(parameters, id)?;

                Ok((space_id, inside))
            }
            Shape::Nifti(_space_id) => Err("Outside-nifti: not yet implemented".to_string()),
        }?;

        complement_helper(core, parameters, space_id, inside)
    }
}

fn filter<'c>(
    core_id: &'c str,
    parameters: &'c CoreQueryParameters<'c>,
    predicate: &'c Option<Predicate>,
    bag: &'c Bag,
) -> mercator_db::ResultSet<'c> {
    match predicate {
        None => bag.execute(core_id, parameters),
        Some(predicate) => bag.filter(predicate, core_id, parameters),
    }
}

fn bag<'c>(
    core_id: &'c str,
    parameters: &'c CoreQueryParameters<'c>,
    bags: &'c [Bag],
) -> mercator_db::ResultSet<'c> {
    let mut results = Vec::new();
    for bag in bags {
        let mut result = bag.execute(core_id, parameters)?;
        results.append(&mut result);
    }

    Ok(results)
}

impl<'e> Executor<'e> for Projection {
    type ResultSet = mercator_db::ResultSet<'e>;

    fn execute(
        &'e self,
        core_id: &'e str,
        parameters: &'e CoreQueryParameters<'e>,
    ) -> Self::ResultSet {
        match self {
            Projection::Nifti(_, _, _bag) => Err("Proj-Nifti: not yet implemented".to_string()),
            Projection::Json(_, _format, bag) => {
                bag.execute(core_id, parameters)
                // FIXME: Add projections here
            }
        }
    }
}

impl<'e> Executor<'e> for Bag {
    type ResultSet = mercator_db::ResultSet<'e>;

    fn execute(
        &'e self,
        core_id: &'e str,
        parameters: &'e CoreQueryParameters<'e>,
    ) -> Self::ResultSet {
        let core = parameters.db.core(core_id)?;

        match self {
            Bag::Distinct(bag) => bag.distinct(core_id, parameters),
            Bag::Filter(predicate, bag) => filter(core_id, parameters, predicate, bag),
            Bag::Complement(bag) => bag.complement(core_id, parameters, core),
            Bag::Intersection(lh, rh) => lh.intersection(core_id, parameters, rh),
            Bag::Union(lh, rh) => lh.union(core_id, parameters, rh),
            Bag::Bag(list) => bag(core_id, parameters, list),
            Bag::Inside(shape) => shape.inside(parameters, core),
            Bag::Outside(shape) => {
                //FIXME: This is currently computed as the complement of the values within the shape, except its surface.
                //       Should this be instead a list of positions within the shape?
                //FIXME: Should we use the Shape's Space to get the maximum bounds or the output Space requested?
                shape.outside(parameters, core)
            }
        }
    }
}
