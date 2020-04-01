(function() {var implementors = {};
implementors["ironsea_index_hashmap"] = [{"text":"impl&lt;F, K&gt; <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"ironsea_index_hashmap/struct.IndexDestructured.html\" title=\"struct ironsea_index_hashmap::IndexDestructured\">Index</a>&lt;F, K&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;K: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html\" title=\"trait core::cmp::Ord\">Ord</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;F: <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;K: <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a>,&nbsp;</span>","synthetic":false,"types":["ironsea_index_hashmap::destructured::Index"]},{"text":"impl&lt;R, K&gt; <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"ironsea_index_hashmap/struct.Index.html\" title=\"struct ironsea_index_hashmap::Index\">Index</a>&lt;R, K&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;K: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html\" title=\"trait core::cmp::Ord\">Ord</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;R: <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;K: <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a>,&nbsp;</span>","synthetic":false,"types":["ironsea_index_hashmap::full_record::Index"]}];
implementors["ironsea_index_sfc_dbc"] = [{"text":"impl&lt;F, K, V&gt; <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"ironsea_index_sfc_dbc/struct.IndexOwned.html\" title=\"struct ironsea_index_sfc_dbc::IndexOwned\">SpaceFillingCurve</a>&lt;F, K, V&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;F: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;K: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html\" title=\"trait core::iter::traits::collect::FromIterator\">FromIterator</a>&lt;V&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html\" title=\"trait core::ops::index::Index\">Index</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>, Output = V&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;V: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html\" title=\"trait core::cmp::Ord\">Ord</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;F: <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;K: <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;V: <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a>,&nbsp;</span>","synthetic":false,"types":["ironsea_index_sfc_dbc::sfc::SpaceFillingCurve"]}];
implementors["mercator_db"] = [{"text":"impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"enum\" href=\"mercator_db/enum.Properties.html\" title=\"enum mercator_db::Properties\">Properties</a>","synthetic":false,"types":["mercator_db::database::db_core::Properties"]},{"text":"impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"mercator_db/struct.Core.html\" title=\"struct mercator_db::Core\">Core</a>","synthetic":false,"types":["mercator_db::database::db_core::Core"]},{"text":"impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"enum\" href=\"mercator_db/space/enum.NumberSet.html\" title=\"enum mercator_db::space::NumberSet\">NumberSet</a>","synthetic":false,"types":["mercator_db::database::space::axis::NumberSet"]},{"text":"impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"mercator_db/space/struct.Graduation.html\" title=\"struct mercator_db::space::Graduation\">Graduation</a>","synthetic":false,"types":["mercator_db::database::space::axis::Graduation"]},{"text":"impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"mercator_db/space/struct.Axis.html\" title=\"struct mercator_db::space::Axis\">Axis</a>","synthetic":false,"types":["mercator_db::database::space::axis::Axis"]},{"text":"impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"enum\" href=\"mercator_db/space/enum.Coordinate.html\" title=\"enum mercator_db::space::Coordinate\">Coordinate</a>","synthetic":false,"types":["mercator_db::database::space::coordinate::Coordinate"]},{"text":"impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"enum\" href=\"mercator_db/space/enum.CoordinateSystem.html\" title=\"enum mercator_db::space::CoordinateSystem\">CoordinateSystem</a>","synthetic":false,"types":["mercator_db::database::space::coordinate_system::CoordinateSystem"]},{"text":"impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"enum\" href=\"mercator_db/space/enum.Position.html\" title=\"enum mercator_db::space::Position\">Position</a>","synthetic":false,"types":["mercator_db::database::space::position::Position"]},{"text":"impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"enum\" href=\"mercator_db/space/enum.Shape.html\" title=\"enum mercator_db::space::Shape\">Shape</a>","synthetic":false,"types":["mercator_db::database::space::shape::Shape"]},{"text":"impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"mercator_db/space/struct.Space.html\" title=\"struct mercator_db::space::Space\">Space</a>","synthetic":false,"types":["mercator_db::database::space::Space"]},{"text":"impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"mercator_db/storage/model/struct.Space.html\" title=\"struct mercator_db::storage::model::Space\">Space</a>","synthetic":false,"types":["mercator_db::storage::model::Space"]},{"text":"impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"mercator_db/storage/model/struct.Axis.html\" title=\"struct mercator_db::storage::model::Axis\">Axis</a>","synthetic":false,"types":["mercator_db::storage::model::Axis"]},{"text":"impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"mercator_db/storage/model/struct.Graduation.html\" title=\"struct mercator_db::storage::model::Graduation\">Graduation</a>","synthetic":false,"types":["mercator_db::storage::model::Graduation"]},{"text":"impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"mercator_db/storage/model/v1/struct.SpatialObject.html\" title=\"struct mercator_db::storage::model::v1::SpatialObject\">SpatialObject</a>","synthetic":false,"types":["mercator_db::storage::model::v1::SpatialObject"]},{"text":"impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"mercator_db/storage/model/v1/struct.Shape.html\" title=\"struct mercator_db::storage::model::v1::Shape\">Shape</a>","synthetic":false,"types":["mercator_db::storage::model::v1::Shape"]},{"text":"impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"mercator_db/storage/model/v2/struct.SpatialObject.html\" title=\"struct mercator_db::storage::model::v2::SpatialObject\">SpatialObject</a>","synthetic":false,"types":["mercator_db::storage::model::v2::SpatialObject"]},{"text":"impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"mercator_db/storage/model/v2/struct.Volume.html\" title=\"struct mercator_db::storage::model::v2::Volume\">Volume</a>","synthetic":false,"types":["mercator_db::storage::model::v2::Volume"]},{"text":"impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"enum\" href=\"mercator_db/storage/model/v2/enum.Shape.html\" title=\"enum mercator_db::storage::model::v2::Shape\">Shape</a>","synthetic":false,"types":["mercator_db::storage::model::v2::Shape"]},{"text":"impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"mercator_db/storage/model/struct.Properties.html\" title=\"struct mercator_db::storage::model::Properties\">Properties</a>","synthetic":false,"types":["mercator_db::storage::model::Properties"]}];
implementors["serde_json"] = [{"text":"impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"serde_json/map/struct.Map.html\" title=\"struct serde_json::map::Map\">Map</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>, <a class=\"enum\" href=\"serde_json/value/enum.Value.html\" title=\"enum serde_json::value::Value\">Value</a>&gt;","synthetic":false,"types":["serde_json::map::Map"]},{"text":"impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"enum\" href=\"serde_json/value/enum.Value.html\" title=\"enum serde_json::value::Value\">Value</a>","synthetic":false,"types":["serde_json::value::Value"]},{"text":"impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"serde_json/struct.Number.html\" title=\"struct serde_json::Number\">Number</a>","synthetic":false,"types":["serde_json::number::Number"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()