#[cfg(test)]
mod parsing {

    /******************************************************************/
    /* FORMATTING DATA                                                */
    /******************************************************************/
    #[cfg(test)]
    mod query {
        use crate::queries;

        fn query_parser() -> queries::QueryParser {
            queries::QueryParser::new()
        }

        #[test]
        fn query() {
            let p = query_parser();

            let nifti = "nifti(inside(point{[0]}))";

            // Option is Empty
            assert!(p.parse("").is_ok());

            // Option is there
            assert!(p.parse(nifti).is_ok());

            // Too many element
            assert!(p.parse(format!("{} {}", nifti, nifti).as_str()).is_err());
        }

        /* Not useful to test this rule
        #[test]
        fn projections() {
            let p = query_parser();

            let nifti = "nifti(point{[0]})";
            let json = "json(., point{[0]})";

            // Each alternative
            assert!(p.parse(nifti).is_ok());
            assert!(p.parse(json).is_ok());
        }
        */

        #[test]
        fn nifti_operator() {
            let p = query_parser();

            // Check allowed forms of the operator
            assert!(p.parse("nifti(inside(point{[0]}))").is_ok());
            assert!(p.parse("nifti(.properties.id, inside(point{[0]}))").is_ok());

            unimplemented!(); // TO REMEMBER SOME WORK IS DUE HERE.

            //FIXME: THIS SHOULD BE ALLOWED
            assert!(p.parse("nifti(2, inside(point{[0]}))").is_ok());
            assert!(p.parse("nifti(2.23, inside(point{[0]}))").is_ok());

            //FIXME: SYNTAX OK, TYPE NOT
            assert!(p.parse("nifti(point{[0], \"space\"})").is_err());
        }

        #[test]
        fn json_operator() {
            let p = query_parser();

            assert!(p.parse("json(true, inside(point{[0]}))").is_ok());
            assert!(p.parse("json(23, inside(point{[0]}))").is_ok());
            assert!(p.parse("json([23, 24], inside(point{[0]}))").is_ok());
            assert!(p.parse("json([23, count(.)], inside(point{[0]}))").is_ok());

            assert!(p.parse("json(true)").is_err());
            assert!(p.parse("json(true,)").is_err());

            assert!(p.parse("json(, inside(point{[0]}))").is_err());
            assert!(p.parse("json(inside(point{[0]}))").is_err());

            assert!(p.parse("json(true, point)").is_err());
        }

        #[test]
        fn json_values() {
            let p = query_parser();

            assert!(p
                .parse(format!("json({}, inside(point{{[0]}}))", "true").as_str())
                .is_ok());
            assert!(p
                .parse(format!("json({}, inside(point{{[0]}}))", "false").as_str())
                .is_ok());
            assert!(p
                .parse(format!("json({}, inside(point{{[0]}}))", "null").as_str())
                .is_ok());

            // Incorrect capitalisation
            assert!(p
                .parse(format!("json({}, inside(point{{[0]}}))", "True").as_str())
                .is_err());
            assert!(p
                .parse(format!("json({}, inside(point{{[0]}}))", "False").as_str())
                .is_err());
            assert!(p
                .parse(format!("json({}, inside(point{{[0]}}))", "Null").as_str())
                .is_err());
        }

        #[test]
        fn json_obj() {
            let p = query_parser();

            assert!(p
                .parse(format!("json({}, inside(point{{[0]}}))", "{}").as_str())
                .is_ok());
            assert!(p
                .parse(format!("json({}, inside(point{{[0]}}))", "{\"field\": 0}").as_str())
                .is_ok());
            assert!(p
                .parse(format!("json({}, inside(point{{[0]}}))", "{\"field\": 0, \"field1\": 1}").as_str())
                .is_ok());
            assert!(p
                .parse(format!("json({}, inside(point{{[0]}}))", "{\"field\": [0, 1]}").as_str())
                .is_ok());
            assert!(p
                .parse(format!("json({}, inside(point{{[0]}}))", "{\"field\": {\"field1\": 0}}").as_str())
                .is_ok());
            assert!(p
                .parse(
                    format!(
                        "json({}, inside(point{{[0]}}))",
                        "{\"field\": [{\"field1\": 0}, {\"field1\": 1}]}"
                    )
                    .as_str()
                )
                .is_ok());
        }

        #[test]
        fn json_pair() {
            let p = query_parser();

            assert!(p
                .parse(format!("json({}, inside(point{{[0]}}))", "{:}").as_str())
                .is_err());
            assert!(p
                .parse(format!("json({}, inside(point{{[0]}}))", "{field: 0}").as_str())
                .is_err());
            assert!(p
                .parse(format!("json({}, inside(point{{[0]}}))", "{0: 0}").as_str())
                .is_err());
            assert!(p
                .parse(format!("json({}, inside(point{{[0]}}))", "{\"0\": }").as_str())
                .is_err());
            assert!(p
                .parse(format!("json({}, inside(point{{[0]}}))", "{\"0\": 0 }").as_str())
                .is_ok());
            assert!(p
                .parse(format!("json({}, inside(point{{[0]}}))", "{\"field\": 0 }").as_str())
                .is_ok());
            assert!(p
                .parse(format!("json({}, inside(point{{[0]}}))", "{\"field\": \"0\" }").as_str())
                .is_ok());
        }

        #[test]
        fn json_array() {
            let p = query_parser();

            assert!(p
                .parse(format!("json({}, inside(point{{[0]}}))", "[, 0]").as_str())
                .is_err());
            assert!(p
                .parse(format!("json({}, inside(point{{[0]}}))", "[]").as_str())
                .is_ok());
            assert!(p
                .parse(format!("json({}, inside(point{{[0]}}))", "[0]").as_str())
                .is_ok());
            assert!(p
                .parse(format!("json({}, inside(point{{[0]}}))", "[0, 1]").as_str())
                .is_ok());
            assert!(p
                .parse(
                    format!("json({}, inside(point{{[0]}}))", "[{\"field\": 0}, {\"field\": 1}]").as_str()
                )
                .is_ok());
        }

        #[test]
        fn aggregations() {
            let p = query_parser();

            // count ()
            assert!(p
                .parse(format!("json({}, inside(point{{[0]}}))", "count()").as_str())
                .is_err());
            assert!(p
                .parse(format!("json({}, inside(point{{[0]}}))", "count(distinct)").as_str())
                .is_err());
            assert!(p
                .parse(format!("json({}, inside(point{{[0]}}))", "count(.)").as_str())
                .is_ok());
            assert!(p
                .parse(format!("json({}, inside(point{{[0]}}))", "count(distinct .)").as_str())
                .is_ok());

            // sum ()
            assert!(p
                .parse(format!("json({}, inside(point{{[0]}}))", "sum()").as_str())
                .is_err());
            assert!(p
                .parse(format!("json({}, inside(point{{[0]}}))", "sum(.)").as_str())
                .is_ok());

            // min ()
            assert!(p
                .parse(format!("json({}, inside(point{{[0]}}))", "min()").as_str())
                .is_err());
            assert!(p
                .parse(format!("json({}, inside(point{{[0]}}))", "min(.)").as_str())
                .is_ok());

            // max ()
            assert!(p
                .parse(format!("json({}, inside(point{{[0]}}))", "max()").as_str())
                .is_err());
            assert!(p
                .parse(format!("json({}, inside(point{{[0]}}))", "max(.)").as_str())
                .is_ok());
        }

        #[test]
        fn json_numbers() {
            let p = query_parser();

            // Integers
            assert!(p
                .parse(format!("json({}, inside(point{{[0]}}))", "0").as_str())
                .is_ok());
            assert!(p
                .parse(format!("json({}, inside(point{{[0]}}))", "+0").as_str())
                .is_err());
            assert!(p
                .parse(format!("json({}, inside(point{{[0]}}))", "-0").as_str())
                .is_ok());
            assert!(p
                .parse(format!("json({}, inside(point{{[0]}}))", "1").as_str())
                .is_ok());
            assert!(p
                .parse(format!("json({}, inside(point{{[0]}}))", "+1").as_str())
                .is_err());
            assert!(p
                .parse(format!("json({}, inside(point{{[0]}}))", "-1").as_str())
                .is_ok());

            // Floating point values
            assert!(p
                .parse(format!("json({}, inside(point{{[0]}}))", "0.0").as_str())
                .is_ok());
            assert!(p
                .parse(format!("json({}, inside(point{{[0]}}))", "+0.0").as_str())
                .is_err());
            assert!(p
                .parse(format!("json({}, inside(point{{[0]}}))", "-0.0").as_str())
                .is_ok());
            assert!(p
                .parse(format!("json({}, inside(point{{[0]}}))", "0.1").as_str())
                .is_ok());
            assert!(p
                .parse(format!("json({}, inside(point{{[0]}}))", "+0.01").as_str())
                .is_err());
            assert!(p
                .parse(format!("json({}, inside(point{{[0]}}))", "-0.01").as_str())
                .is_ok());
        }
    }

    #[cfg(test)]
    mod filters {
        use crate::queries;

        /******************************************************************/
        /* SELECTING / FILTERING DATA                                     */
        /******************************************************************/
        fn filters_parser() -> queries::FiltersParser {
            queries::FiltersParser::new()
        }

        #[test]
        fn filters() {
            let p = filters_parser();

            assert!(p.parse("").is_err());

            assert!(p.parse("inside(point{[0]})").is_ok());
        }

        /* Not useful to test this rule
        #[test]
        fn bags() {
            let p = filters_parser();
        } */

        #[test]
        fn distinct() {
            let p = filters_parser();

            assert!(p.parse("distinct()").is_err());

            assert!(p.parse("distinct(inside(point{[0]}))").is_ok());
        }

        #[test]
        fn complement() {
            let p = filters_parser();

            assert!(p.parse("complement()").is_err());

            assert!(p.parse("complement(inside(point{[0]}))").is_ok());
        }

        #[test]
        fn intersection() {
            let p = filters_parser();

            assert!(p.parse("intersection()").is_err());
            assert!(p.parse("intersection(inside(point{[0]}))").is_err());
            assert!(p
                .parse("intersection(inside(point{[0]}), inside(point{[0]}), inside(point{[0]}))")
                .is_err());

            assert!(p.parse("intersection(inside(point{[0]}), inside(point{[0]}))").is_ok());
        }

        #[test]
        fn union() {
            let p = filters_parser();

            assert!(p.parse("union()").is_err());
            assert!(p.parse("union(inside(point{[0]}))").is_err());
            assert!(p
                .parse("union(inside(point{[0]}), inside(point{[0]}), inside(point{[0]}))")
                .is_err());

            assert!(p.parse("union(inside(point{[0]}), inside(point{[0]}))").is_ok());
        }

        #[test]
        fn filter() {
            let p = filters_parser();

            assert!(p.parse("filter()").is_err());
            assert!(p.parse("filter(inside(point{[0]}))").is_ok());
            assert!(p.parse("filter(=(., [0]))").is_ok());

            assert!(p.parse("filter(=(., [0]), inside(point{[0]}))").is_ok());
        }

        /* Not useful to test this rule
        #[test]
        fn predicates() {
            let p = filters_parser();
        }*/

        #[test]
        fn less() {
            let p = filters_parser();

            assert!(p
                .parse(format!("filter({}, inside(point{{[0]}}))", "<(., [0])").as_str())
                .is_ok());

            assert!(p
                .parse(format!("filter({}, inside(point{{[0]}}))", "<(, [0])").as_str())
                .is_err());
            assert!(p
                .parse(format!("filter({}, inside(point{{[0]}}))", "<(.)").as_str())
                .is_err());
            assert!(p
                .parse(format!("filter({}, inside(point{{[0]}}))", "<()").as_str())
                .is_err());
        }

        #[test]
        fn greater() {
            let p = filters_parser();

            assert!(p
                .parse(format!("filter({}, inside(point{{[0]}}))", ">(., [0])").as_str())
                .is_ok());

            assert!(p
                .parse(format!("filter({}, inside(point{{[0]}}))", ">(, [0])").as_str())
                .is_err());
            assert!(p
                .parse(format!("filter({}, inside(point{{[0]}}))", ">(.)").as_str())
                .is_err());
            assert!(p
                .parse(format!("filter({}, inside(point{{[0]}}))", ">()").as_str())
                .is_err());
        }

        #[test]
        fn equal() {
            let p = filters_parser();

            assert!(p
                .parse(format!("filter({}, inside(point{{[0]}}))", "=(., [0])").as_str())
                .is_ok());

            assert!(p
                .parse(format!("filter({}, inside(point{{[0]}}))", "=(, [0])").as_str())
                .is_err());
            assert!(p
                .parse(format!("filter({}, inside(point{{[0]}}))", "=(.)").as_str())
                .is_err());
            assert!(p
                .parse(format!("filter({}, inside(point{{[0]}}))", "=()").as_str())
                .is_err());
        }

        #[test]
        fn not() {
            let p = filters_parser();

            assert!(p
                .parse(format!("filter({}, inside(point{{[0]}}))", "!(=(., [0]))").as_str())
                .is_ok());

            assert!(p
                .parse(format!("filter({}, inside(point{{[0]}}))", "!()").as_str())
                .is_err());
        }

        #[test]
        fn and() {
            let p = filters_parser();

            assert!(p
                .parse(format!("filter({}, inside(point{{[0]}}))", "&(=(., [0]), =(., [0]))").as_str())
                .is_ok());

            assert!(p
                .parse(format!("filter({}, inside(point{{[0]}}))", "&(, =(., [0]))").as_str())
                .is_err());
            assert!(p
                .parse(format!("filter({}, inside(point{{[0]}}))", "&(|(=(., [0])))").as_str())
                .is_err());
            assert!(p
                .parse(format!("filter({}, inside(point{{[0]}}))", "&()").as_str())
                .is_err());
        }

        #[test]
        fn or() {
            let p = filters_parser();

            assert!(p
                .parse(format!("filter({}, inside(point{{[0]}}))", "|(=(., [0]), =(., [0]))").as_str())
                .is_ok());

            assert!(p
                .parse(format!("filter({}, inside(point{{[0]}}))", "|(, =(., [0]))").as_str())
                .is_err());
            assert!(p
                .parse(format!("filter({}, inside(point{{[0]}}))", "|(|(=(., [0])))").as_str())
                .is_err());
            assert!(p
                .parse(format!("filter({}, inside(point{{[0]}}))", "|()").as_str())
                .is_err());
        }

        #[test]
        fn bag() {
            let p = filters_parser();

            assert!(p.parse("bag{}").is_err());

            assert!(p.parse("bag{inside(point{[0]})}").is_ok());
            assert!(p.parse("bag{inside(point{[0]}), inside(point{[0]})}").is_ok());
            assert!(p.parse("bag{inside(point{[0]}), inside(point{[0]}), inside(point{[0]})}").is_ok());
            assert!(p
                .parse("bag{inside(point{[0]}), inside(hypersphere{[0], 1}), inside(hyperrectangle{[0], [1]})}")
                .is_ok());
        }

        #[test]
        fn outside() {
            let p = filters_parser();

            assert!(p.parse("outside()").is_err());

            assert!(p.parse("outside(point{[0]})").is_ok());
        }

        #[test]
        fn inside() {
            let p = filters_parser();

            assert!(p.parse("inside()").is_err());

            assert!(p.parse("inside(point{[0]})").is_ok());
        }

        /* Not useful to test this rule
        #[test]
        fn shapes() {
            let p = filters_parser();

            assert!(p.parse("point{[0]}").is_ok());
            assert!(p.parse("hyperrectangle{[0], [1]}").is_ok());
            assert!(p.parse("hypersphere{[0], 1}").is_ok());
            assert!(p.parse("nifti{\"\", uri(\"\")}").is_ok());
        }*/

        #[test]
        fn hyperrectangle() {
            let p = filters_parser();

            // At least two positions when it is aligned with the axis, otherwise an even number
            // of positions, as the number of vertices follows the rule 2**k, where k is the number
            // of dimensions of the space containing the hyperrectangle.
            assert!(p.parse("inside(hyperrectangle{})").is_err());
            assert!(p.parse("inside(hyperrectangle{[]})").is_err());
            assert!(p.parse("inside(hyperrectangle{[0]})").is_err());
            assert!(p.parse("inside(hyperrectangle{[0], [1], [2]})").is_err());
            assert!(p.parse("inside(hyperrectangle{[0], [1], [2], [3], [4]})").is_err());

            assert!(p.parse("inside(hyperrectangle{[0], [1]})").is_ok());
            assert!(p.parse("inside(hyperrectangle{[0], [1], \"space\"})").is_ok());
            assert!(p.parse("inside(hyperrectangle{[0], [1], [2], [3]})").is_ok());
            assert!(p.parse("inside(hyperrectangle{[0], [1], [2], [3]})").is_ok());
            assert!(p
                .parse("inside(hyperrectangle{[0], [1], [2], [3], [4], [5]})")
                .is_ok());
            assert!(p
                .parse("inside(hyperrectangle{[0], [1], [2], [3], [4], [5], \"space\"})")
                .is_ok());
        }

        #[test]
        fn hyperrsphere() {
            let p = filters_parser();

            assert!(p.parse("inside(hypersphere{}").is_err());
            assert!(p.parse("inside(hypersphere{[]})").is_err());
            assert!(p.parse("inside(hypersphere{[0]})").is_err());

            assert!(p.parse("inside(hypersphere{[0], 23})").is_ok());
            assert!(p.parse("inside(hypersphere{[0], 23, \"space\"})").is_ok());
        }

        #[test]
        fn point() {
            let p = filters_parser();

            assert!(p.parse("inside(point{})").is_err());
            assert!(p.parse("inside(point{[]})").is_err());

            assert!(p.parse("inside(point{[0]})").is_ok());
            assert!(p.parse("inside(point{[0], \"space\"})").is_ok());
        }

        #[test]
        fn nifti() {
            let _p = filters_parser();
            unimplemented!();
        }

        #[test]
        fn byte_provider() {
            let _p = filters_parser();
            unimplemented!();
        }

        /* Not useful to test this rule
        #[test]
        fn positions() {
            let p = filters_parser();

            assert!(p
                .parse(
                    format!(
                        "filter(=({}, [1]), inside(point{{[0]}}))",
                        "str_cmp_ignore_case(.field, \"\")"
                    )
                    .as_str()
                )
                .is_ok());
            assert!(p
                .parse(format!("filter(=({}, [1]), inside(point{{[0]}}))", "str_cmp(.field, \"\")").as_str())
                .is_ok());
            assert!(p
                .parse(format!("filter(=({}, [1]), inside(point{{[0]}}))", ".field").as_str())
                .is_ok());
            assert!(p
                .parse(format!("filter(=({}, [1]), inside(point{{[0]}}))", "[0]").as_str())
                .is_ok());

            assert!(p
                .parse(format!("filter(=({}, [1]), inside(point{{[0]}}))", "inside(point{[0]})").as_str())
                .is_err());
            assert!(p
                .parse(format!("filter(=({}, [1]), inside(point{{[0]}}))", "{0}").as_str())
                .is_err());
            assert!(p
                .parse(format!("filter(=({}, [1]), inside(point{{[0]}}))", "").as_str())
                .is_err());
        }*/

        #[test]
        fn str_cmp() {
            let p = filters_parser();

            assert!(p
                .parse(
                    format!("filter(=({}, [1]), inside(point{{[0]}}))", "str_cmp(.field, \"\")").as_str()
                )
                .is_ok());

            assert!(p
                .parse(format!("filter(=({}, [1]), inside(point{{[0]}}))", "str_cmp(.field)").as_str())
                .is_err());
            assert!(p
                .parse(format!("filter(=({}, [1]), inside(point{{[0]}}))", "str_cmp(\"\")").as_str())
                .is_err());
        }

        #[test]
        fn str_cmp_icase() {
            let p = filters_parser();

            assert!(p
                .parse(
                    format!(
                        "filter(=({}, [1]), inside(point{{[0]}}))",
                        "str_cmp_ignore_case(.field, \"\")"
                    )
                    .as_str()
                )
                .is_ok());

            assert!(p
                .parse(
                    format!(
                        "filter(=({}, [1]), inside(point{{[0]}}))",
                        "str_cmp_ignore_case(.field)"
                    )
                    .as_str()
                )
                .is_err());
            assert!(p
                .parse(
                    format!(
                        "filter(=({}, [1]), inside(point{{[0]}}))",
                        "str_cmp_ignore_case(\"\")"
                    )
                    .as_str()
                )
                .is_err());
        }

        #[test]
        fn selector() {
            let p = filters_parser();

            assert!(p
                .parse(format!("filter(=({}, [1]), inside(point{{[0]}}))", ".").as_str())
                .is_ok());
            assert!(p
                .parse(format!("filter(=({}, [1]), inside(point{{[0]}}))", ".field").as_str())
                .is_ok());
            assert!(p
                .parse(format!("filter(=({}, [1]), inside(point{{[0]}}))", ".field.field").as_str())
                .is_ok());
            assert!(p
                .parse(format!("filter(=({}, [1]), inside(point{{[0]}}))", ".field[1].field").as_str())
                .is_ok());
            assert!(p
                .parse(format!("filter(=({}, [1]), inside(point{{[0]}}))", ".field.field[1]").as_str())
                .is_ok());
        }

        #[test]
        fn position() {
            let p = filters_parser();

            // Empty
            assert!(p.parse(format!("inside(point{{{}}})", "[]").as_str()).is_err());

            // Non-numerical coordinate:
            assert!(p.parse(format!("inside(point{{{}}})", "[aa]").as_str()).is_err());

            assert!(p
                .parse(format!("inside(point{{{}}})", "[\"aa\"]").as_str())
                .is_err());

            // One or more coordinates
            assert!(p.parse(format!("inside(point{{{}}})", "[0]").as_str()).is_ok());
            assert!(p.parse(format!("inside(point{{{}}})", "[0, 0]").as_str()).is_ok());
            assert!(p
                .parse(format!("inside(point{{{}}})", "[0, 0, 0]").as_str())
                .is_ok());
            assert!(p
                .parse(format!("inside(point{{{}}})", "[0, 0, 0, 0]").as_str())
                .is_ok());
            assert!(p
                .parse(format!("inside(point{{{}}})", "[0,0,0,0]").as_str())
                .is_ok());
        }

        #[test]
        fn field() {
            let p = filters_parser();

            // Single dot
            assert!(p
                .parse(format!("filter(<({}, [1]), inside(point{{[0]}}))", ".").as_str())
                .is_ok());

            // Check first character is within allowed characters
            assert!(p
                .parse(format!("filter(<({}, [1]), inside(point{{[0]}}))", ".a").as_str())
                .is_ok());
            assert!(p
                .parse(format!("filter(<({}, [1]), inside(point{{[0]}}))", "._").as_str())
                .is_ok());
            assert!(p
                .parse(format!("filter(<({}, [1]), inside(point{{[0]}}))", ".2").as_str())
                .is_err());

            // Check second character is within allowed characters
            assert!(p
                .parse(format!("filter(<({}, [1]), inside(point{{[0]}}))", ".fa").as_str())
                .is_ok());
            assert!(p
                .parse(format!("filter(<({}, [1]), inside(point{{[0]}}))", ".f2").as_str())
                .is_ok());
            assert!(p
                .parse(format!("filter(<({}, [1]), inside(point{{[0]}}))", ".f_").as_str())
                .is_ok());
            assert!(p
                .parse(format!("filter(<({}, [1]), inside(point{{[0]}}))", ".f2").as_str())
                .is_ok());

            // Check we can add subscript
            assert!(p
                .parse(format!("filter(<({}, [1]), inside(point{{[0]}}))", ".[23]").as_str())
                .is_ok());
            assert!(p
                .parse(format!("filter(<({}, [1]), inside(point{{[0]}}))", ".f[0]").as_str())
                .is_ok());
            assert!(p
                .parse(format!("filter(<({}, [1]), inside(point{{[0]}}))", ".f[2]").as_str())
                .is_ok());
            assert!(p
                .parse(format!("filter(<({}, [1]), inside(point{{[0]}}))", ".f[23]").as_str())
                .is_ok());

            // Invalid index values
            assert!(p
                .parse(format!("filter(<({}, [1]), inside(point{{[0]}}))", ".f[2.3]").as_str())
                .is_err());
            assert!(p
                .parse(format!("filter(<({}, [1]), inside(point{{[0]}}))", ".f[02]").as_str())
                .is_err());
            assert!(p
                .parse(format!("filter(<({}, [1]), inside(point{{[0]}}))", ".f[-2]").as_str())
                .is_err());
            assert!(p
                .parse(format!("filter(<({}, [1]), inside(point{{[0]}}))", ".f[2e2]").as_str())
                .is_err());
            assert!(p
                .parse(format!("filter(<({}, [1]), inside(point{{[0]}}))", ".f[2E2]").as_str())
                .is_err());
            assert!(p
                .parse(format!("filter(<({}, [1]), inside(point{{[0]}}))", ".f[+2]").as_str())
                .is_err());
        }

        #[test]
        fn string() {
            fn test_str_ok(p: &queries::FiltersParser, string: &str) {
                let n = format!(
                    "{}{}{}",
                    "nifti{uri(\"http://a.nifti.file\"), ", string, " }"
                );
                let n = n.as_str();

                assert!(p.parse(n).is_ok());
            }

            fn test_str_err(p: &queries::FiltersParser, string: &str) {
                let n = format!(
                    "{}{}{}",
                    "nifti{", string, ", uri(\"http://a.nifti.file\") }"
                );
                let n = n.as_str();

                assert!(p.parse(n).is_err());
            }

            let p = &filters_parser();

            // Empty String
            test_str_ok(p, r#""""#);

            // Usual escapes
            test_str_ok(p, r#""\"""#);
            test_str_ok(p, r#""\\""#);
            test_str_ok(p, r#""\/""#);
            test_str_ok(p, r#""\b""#);
            test_str_ok(p, r#""\f""#);
            test_str_ok(p, r#""\n""#);
            test_str_ok(p, r#""\r""#);
            test_str_ok(p, r#""\t""#);

            // Unicode Escape
            test_str_ok(p, r#""\u0012""#);
            test_str_ok(p, r#""\u001F""#);
            test_str_ok(p, r#""\u001a""#);

            // ASCI Letters & digit
            test_str_ok(p, r#""abcdefghijklmnopqrstuvwxyz""#);
            test_str_ok(p, r#""ABCDEFGHIJKLMNOPQRSTUVWXYZ""#);
            test_str_ok(p, r#""0123456789""#);

            // Space and some non-white characters
            test_str_ok(p, r#"" ,.-;:!?'^&|§°+*ç%_""#);

            // Invalid
            test_str_err(p, "\"\u{0010}\""); // rust requires \u{..}, while JSON does not.
        }

        #[test]
        fn positive_numbers() {
            let p = filters_parser();

            // Integers
            assert!(p
                .parse(format!("inside(hypersphere{{[0],{}}})", "0").as_str())
                .is_ok());
            assert!(p
                .parse(format!("inside(hypersphere{{[0],{}}})", "+0").as_str())
                .is_ok());
            assert!(p
                .parse(format!("inside(hypersphere{{[0],{}}})", "-0").as_str())
                .is_err());
            assert!(p
                .parse(format!("inside(hypersphere{{[0],{}}})", "1").as_str())
                .is_ok());
            assert!(p
                .parse(format!("inside(hypersphere{{[0],{}}})", "+1").as_str())
                .is_ok());
            assert!(p
                .parse(format!("inside(hypersphere{{[0],{}}})", "-1").as_str())
                .is_err());

            // Floating point values
            assert!(p
                .parse(format!("inside(hypersphere{{[0],{}}})", "0.0").as_str())
                .is_ok());
            assert!(p
                .parse(format!("inside(hypersphere{{[0],{}}})", "+0.0").as_str())
                .is_ok());
            assert!(p
                .parse(format!("inside(hypersphere{{[0],{}}})", "-0.0").as_str())
                .is_err());
            assert!(p
                .parse(format!("inside(hypersphere{{[0],{}}})", "0.1").as_str())
                .is_ok());
            assert!(p
                .parse(format!("inside(hypersphere{{[0],{}}})", "+0.01").as_str())
                .is_ok());
            assert!(p
                .parse(format!("inside(hypersphere{{[0],{}}})", "-0.01").as_str())
                .is_err());
        }

        #[test]
        fn numbers() {
            let p = filters_parser();

            // Integers
            assert!(p.parse(format!("inside(point{{[{}]}})", "0").as_str()).is_ok());
            assert!(p.parse(format!("inside(point{{[{}]}})", "+0").as_str()).is_ok());
            assert!(p.parse(format!("inside(point{{[{}]}})", "-0").as_str()).is_ok());
            assert!(p.parse(format!("inside(point{{[{}]}})", "1").as_str()).is_ok());
            assert!(p.parse(format!("inside(point{{[{}]}})", "+1").as_str()).is_ok());
            assert!(p.parse(format!("inside(point{{[{}]}})", "-1").as_str()).is_ok());

            // Floating point values
            assert!(p.parse(format!("inside(point{{[{}]}})", "0.0").as_str()).is_ok());
            assert!(p.parse(format!("inside(point{{[{}]}})", "+0.0").as_str()).is_ok());
            assert!(p.parse(format!("inside(point{{[{}]}})", "-0.0").as_str()).is_ok());
            assert!(p.parse(format!("inside(point{{[{}]}})", "0.1").as_str()).is_ok());
            assert!(p.parse(format!("inside(point{{[{}]}})", "+0.01").as_str()).is_ok());
            assert!(p.parse(format!("inside(point{{[{}]}})", "-0.01").as_str()).is_ok());
        }

        #[test]
        fn num() {
            let p = filters_parser();

            // Integers
            assert!(p.parse(format!("inside(point{{[{}]}})", "0").as_str()).is_ok());
            assert!(p.parse(format!("inside(point{{[{}]}})", "1").as_str()).is_ok());
            assert!(p.parse(format!("inside(point{{[{}]}})", "1e2").as_str()).is_ok());
            assert!(p.parse(format!("inside(point{{[{}]}})", "1e+2").as_str()).is_ok());
            assert!(p.parse(format!("inside(point{{[{}]}})", "1e-2").as_str()).is_ok());
            assert!(p.parse(format!("inside(point{{[{}]}})", "1E2").as_str()).is_ok());
            assert!(p.parse(format!("inside(point{{[{}]}})", "100").as_str()).is_ok());

            assert!(p.parse(format!("inside(point{{[{}]}})", "010").as_str()).is_err());

            // Floating point values (normalized)
            assert!(p.parse(format!("inside(point{{[{}]}})", "0.0").as_str()).is_ok());
            assert!(p.parse(format!("inside(point{{[{}]}})", "0.1").as_str()).is_ok());
            assert!(p.parse(format!("inside(point{{[{}]}})", "0.1e0").as_str()).is_ok());
            assert!(p.parse(format!("inside(point{{[{}]}})", "0.1e2").as_str()).is_ok());
            assert!(p.parse(format!("inside(point{{[{}]}})", "0.1e+2").as_str()).is_ok());
            assert!(p.parse(format!("inside(point{{[{}]}})", "0.1e-2").as_str()).is_ok());
            assert!(p.parse(format!("inside(point{{[{}]}})", "0.1E2").as_str()).is_ok());
            assert!(p.parse(format!("inside(point{{[{}]}})", "0.1E23").as_str()).is_ok());
            assert!(p.parse(format!("inside(point{{[{}]}})", "0.01").as_str()).is_ok());

            assert!(p.parse(format!("inside(point{{[{}]}})", "0.").as_str()).is_err());
            assert!(p
                .parse(format!("inside(point{{[{}]}})", "0.1E03").as_str())
                .is_err());
            assert!(p
                .parse(format!("inside(point{{[{}]}})", "0.1E0.3").as_str())
                .is_err());

            // Floating point values (denormalized)
            assert!(p.parse(format!("inside(point{{[{}]}})", "1.0").as_str()).is_ok());
            assert!(p.parse(format!("inside(point{{[{}]}})", "1.1").as_str()).is_ok());
            assert!(p.parse(format!("inside(point{{[{}]}})", "1.1e0").as_str()).is_ok());
            assert!(p.parse(format!("inside(point{{[{}]}})", "1.1e2").as_str()).is_ok());
            assert!(p.parse(format!("inside(point{{[{}]}})", "1.1e+2").as_str()).is_ok());
            assert!(p.parse(format!("inside(point{{[{}]}})", "1.1e-2").as_str()).is_ok());
            assert!(p.parse(format!("inside(point{{[{}]}})", "1.1E2").as_str()).is_ok());
            assert!(p.parse(format!("inside(point{{[{}]}})", "1.1E23").as_str()).is_ok());
            assert!(p.parse(format!("inside(point{{[{}]}})", "1.01").as_str()).is_ok());
            assert!(p.parse(format!("inside(point{{[{}]}})", "10.1").as_str()).is_ok());

            assert!(p.parse(format!("inside(point{{[{}]}})", "1.").as_str()).is_err());
            assert!(p.parse(format!("inside(point{{[{}]}})", "01.1").as_str()).is_err());
            assert!(p
                .parse(format!("inside(point{{[{}]}})", "1.1E03").as_str())
                .is_err());
            assert!(p
                .parse(format!("inside(point{{[{}]}})", "1.1E0.3").as_str())
                .is_err());
        }
    }
}
