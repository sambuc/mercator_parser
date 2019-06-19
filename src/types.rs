#[derive(Clone, Debug)]
pub enum LiteralTypes {
    String,
    Int,
    Float,
    Bag(Vec<LiteralTypes>),          // List of types (heterogeneous)
    Vector(Vec<LiteralTypes>),       // List of coordinates types (heterogeneous)
    Array(usize, Box<LiteralTypes>), // Length, homogeneous type
}

impl PartialEq for LiteralTypes {
    fn eq(&self, other: &Self) -> bool {
        match self {
            LiteralTypes::String => match other {
                LiteralTypes::String => true,
                _ => false,
            },
            LiteralTypes::Int => match other {
                LiteralTypes::Int => true,
                _ => false,
            },
            LiteralTypes::Float => match other {
                LiteralTypes::Float => true,
                LiteralTypes::Int => true,
                _ => false,
            },
            LiteralTypes::Bag(_) => match other {
                LiteralTypes::Bag(_) => true,
                _ => false,
            },
            LiteralTypes::Vector(v) => match other {
                LiteralTypes::Vector(ov) => {
                    let n = v.len();
                    if n != ov.len() {
                        false
                    } else {
                        for i in 0..n {
                            if v[i] != ov[i] {
                                return false;
                            }
                        }
                        true
                    }
                }
                _ => false,
            },
            LiteralTypes::Array(n, t) => match other {
                LiteralTypes::Array(on, ot) => {
                    if on == n {
                        t == ot
                    } else {
                        false
                    }
                }
                _ => false,
            },
        }
    }
}
