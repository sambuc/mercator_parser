use mercator_db::DataBase;

use super::expressions::Predictor;
use super::symbols::*;

impl Predictor for Projection {
    fn predict(&self, db: &DataBase) -> Result<f64, String> {
        match self {
            Projection::Nifti(_, _, bag) => bag.predict(db),
            Projection::JSON(_, _, bag) => bag.predict(db),
        }
    }
}

impl Predictor for Bag {
    fn predict(&self, db: &DataBase) -> Result<f64, String> {
        match self {
            Bag::Distinct(bag) => bag.predict(db),
            Bag::Filter(_, bag) => bag.predict(db),
            Bag::Complement(bag) => Ok(db.space(bag.space())?.volume() - bag.predict(db)?),
            Bag::Intersection(lh, rh) => {
                let l = lh.predict(db)?;
                let r = rh.predict(db)?;
                if l < r {
                    Ok(l)
                } else {
                    Ok(r)
                }
            }
            Bag::Union(lh, rh) => Ok(lh.predict(db)? + rh.predict(db)?),
            Bag::Bag(bags) => {
                let mut s = 0.0;
                for bag in bags {
                    s += bag.predict(db)?;
                }
                Ok(s)
            }
            Bag::Inside(shape) => shape.predict(db),
            Bag::Outside(shape) => Ok(db.space(shape.space())?.volume() - shape.predict(db)?),
        }
    }
}

impl Predictor for Shape {
    fn predict(&self, _db: &DataBase) -> Result<f64, String> {
        Ok(self.volume())
    }
}
