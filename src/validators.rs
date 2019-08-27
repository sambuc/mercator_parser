use super::expressions::Validator;
use super::symbols::*;

pub type ValidationResult = Result<LiteralTypes, String>;

impl Validator for Projection {
    type ValidationResult = self::ValidationResult;

    fn validate(&self) -> ValidationResult {
        match self {
            Projection::Nifti(_, _, _) => Err("not yet implemented".to_string()),
            Projection::JSON(_, _format, bag) => bag.validate(),
            //FIXME: Add support for projections
            /* match format.validate() {
                Ok(_) => bag.validate(),
                Err(_) => Err(()),
            }*/
        }
    }
}

impl Validator for Bag {
    type ValidationResult = self::ValidationResult;

    fn validate(&self) -> ValidationResult {
        fn compare_bag_types(lh: &Bag, rh: &Bag) -> ValidationResult {
            if lh.space().cmp(rh.space()) != std::cmp::Ordering::Equal {
                return Err(format!(
                    "left and right sets are defined in different reference spaces: '{}' vs '{}'.",
                    lh.space(),
                    rh.space()
                ));
            }

            let l = lh.validate();
            let r = rh.validate();

            match &l {
                Err(_) => l,
                Ok(tl) => match r {
                    e @ Err(_) => e,
                    Ok(tr) => {
                        if tl != &tr {
                            Err(format!(
                                "Incoherent types between left and right sets: '{:?}' vs '{:?}'",
                                tl, &tr
                            ))
                        } else {
                            l
                        }
                    }
                },
            }
        }

        match self {
            Bag::Distinct(bag) => bag.validate(),
            Bag::Filter(_, bag) => bag.validate(),
            Bag::Complement(bag) => bag.validate(),
            Bag::Intersection(lh, rh) => compare_bag_types(lh, rh),
            Bag::Union(lh, rh) => compare_bag_types(lh, rh),
            Bag::Bag(bags) => {
                for b in bags {
                    let t = b.validate();
                    if t.is_err() {
                        return t;
                    }
                }

                Ok(get_type())
            }
            Bag::Inside(shape) => shape.validate(),
            Bag::Outside(shape) => shape.validate(),
        }
    }
}

impl Validator for Shape {
    type ValidationResult = self::ValidationResult;

    fn validate(&self) -> ValidationResult {
        match self {
            Shape::Point(_, v) => v.validate(),
            Shape::HyperRectangle(_space, pos) => {
                let first = pos[0].get_type();
                match pos.len() {
                    2 => {
                        if first != pos[1].get_type() {
                            Err(format!(
                                "HyperRectangle: Incompatible types in points definitions: '{:?}' vs '{:?}'",
                                first,
                                pos[1].get_type()
                            ))
                        } else {
                            Ok(first)
                        }
                    }
                    _ => {
                        //FIXME: Implement arbitrary hypercube definition support. For now reject.
                        Err("not yet implemented".to_string())
                        /*
                        fn check_orthogonal(pos: &Vec<LiteralPosition>) -> bool {
                            let k = pos.len();
                            let mut raw_pos = vec![];

                            for s in pos {
                                match s {
                                    LiteralTypes::Vector(n) => {
                                        let mut v = vec![];
                                        for c in n {
                                            v.push(match c {
                                                LiteralNumber::Int(x) => x as f64,
                                                LiteralNumber::Float(x) => x,
                                            });
                                        }
                                        raw_pos.push(v)
                                    }
                                    _ => pass,
                                }
                            }

                            for p in raw_pos {}
                            true
                        }
                        match &first {
                            LiteralTypes::Vector(n) => {
                                if 2usize.pow(n.len() as u32) == pos.len() {
                                    // Check we have coherent types for the coordinates.
                                    for point in pos {
                                        if first != point.get_type() {
                                            return Err(());
                                        }
                                    }

                                    // We need to check the points define a shape with orthogonal faces.
                                    if check_orthogonal(pos) {
                                        Ok(first)
                                    } else {
                                        Err(())
                                    }
                                } else {
                                    Err(())
                                }
                            }
                            _ => Err(()),
                        }*/
                    }
                }
            }
            Shape::HyperSphere(_, pos, _) => pos.validate(),
            Shape::Nifti(_) => Err("not yet implemented".to_string()),
        }
    }
}

impl Validator for LiteralPosition {
    type ValidationResult = self::ValidationResult;

    fn validate(&self) -> ValidationResult {
        Ok(self.get_type())
    }
}

impl Validator for LiteralSelector {
    type ValidationResult = self::ValidationResult;

    fn validate(&self) -> ValidationResult {
        Ok(self.get_type())
    }
}
