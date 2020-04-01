#![forbid(unsafe_code)]

//! # Mercator Parser
//!
//! Query parser for Mercator.
//!
//! ## Mercator: Spatial Index
//!
//! **Mercator** is a spatial *volumetric* index for the
//! [Human Brain Project]. It is a component of the [Knowledge Graph]
//! service, which  provides the spatial anchoring for the metadata
//! registered as well as processes the volumetric queries.
//!
//! It is build on top of the Iron Sea database toolkit.
//!
//! ## Iron Sea: Database Toolkit
//! **Iron Sea** provides a set of database engine bricks, which can be
//! combined and applied on arbitrary data structures.
//!
//! Unlike a traditional database, it does not assume a specific
//! physical structure for the tables nor the records, but relies on the
//! developer to provide a set of extractor functions which are used by
//! the specific indices provided.
//!
//! This enables the index implementations to be agnostic from the
//! underlying data structure, and re-used.
//!
//! [Human Brain Project]: http://www.humanbrainproject.eu
//! [Knowledge Graph]: http://www.humanbrainproject.eu/en/explore-the-brain/search/

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(#[allow(clippy::all,unused_parens)] pub queries); // synthesized by LALRPOP

// Note: We do not enable for the whole library deny(missing_docs), as
//       it requires the automatically generated parser to be documented
//       as well.
//       Instead we enable it per modules below, except for the tests.

#[warn(missing_docs)]
mod evaluators;
#[warn(missing_docs)]
mod executors;
#[warn(missing_docs)]
mod expressions;
#[warn(missing_docs)]
mod predictors;
#[warn(missing_docs)]
mod validators;

#[warn(missing_docs)]
mod symbols;
#[warn(missing_docs)]
mod types;

pub use expressions::Executor;
pub use expressions::Predictor;
pub use expressions::Validator;
pub use queries::FiltersParser;
pub use queries::QueryParser;
pub use validators::ValidationResult;

#[cfg(test)]
mod tests;
