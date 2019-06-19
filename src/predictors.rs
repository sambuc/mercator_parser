use super::expression::Predictor;
use super::symbols::*;

use super::database::space;

impl Predictor for Projection {
    fn predict(&self) -> f64 {
        match self {
            Projection::Nifti(_, _, bag) => bag.predict(),
            Projection::JSON(_, _, bag) => bag.predict(),
        }
    }
}

impl Predictor for Bag {
    fn predict(&self) -> f64 {
        match self {
            Bag::Distinct(bag) => bag.predict(),
            Bag::Filter(_, bag) => bag.predict(),
            Bag::Complement(bag) => space::max_volume(bag.space()) - bag.predict(),
            Bag::Intersection(lh, rh) => {
                let l = lh.predict();
                let r = rh.predict();
                if l < r {
                    l
                } else {
                    r
                }
            }
            Bag::Union(lh, rh) => lh.predict() + rh.predict(),
            Bag::Bag(bags) => {
                let mut s = 0.0;
                for bag in bags {
                    s += bag.predict();
                }
                s
            }
            Bag::Inside(shape) => shape.predict(),
            Bag::Outside(shape) => space::max_volume(shape.space()) - shape.predict(),
        }
    }
}

impl Predictor for Shape {
    fn predict(&self) -> f64 {
        self.volume()
    }
}
