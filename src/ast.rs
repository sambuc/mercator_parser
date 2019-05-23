trait Expression {
    fn check_type();
    fn predict();
    fn execute();
}

/**********************************************************************/
/* FORMATTING DATA                                                    */
/**********************************************************************/
#[derive(Clone, Debug)]
pub enum Projection {
    Nifti(LiteralSelector, Bag),
    JSON(JsonValue, Bag),
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
    Filter(Option<Predicate>, Option<Box<Bag>>),
    Complement(Box<Bag>),
    Intersection(Box<Bag>, Box<Bag>),
    Union(Box<Bag>, Box<Bag>),
    Bag(Vec<Bag>),
    Inside(Shape),
    Outside(Shape),
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
    Point(LiteralPosition),
    HyperRectangle(Vec<LiteralPosition>),
    HyperSphere(LiteralPosition, LiteralNumber),
    Nifti(),
}

/**********************************************************************/
/* POSITIONS                                                          */
/**********************************************************************/
#[derive(Clone, Debug)]
pub enum Position {
    StrCmpICase(LiteralSelector, String),
    StrCmp(LiteralSelector, String),
    Selector(LiteralSelector),
    LiteralPosition(LiteralPosition),
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

pub type LiteralPosition = Vec<LiteralNumber>;
pub type LiteralSelector = Vec<Field>;
