grammar queries;
import filters;

/**********************************************************************/
/* FORMATTING DATA                                                    */
/**********************************************************************/
queries
    :  projection_operators?
    ;

projection_operators
    : nifti_operator
    | json_operator
    ;

/* If selector is not provided, one (1) will be used as the values for
 * each position where there is a point in bag_expression.
 *
 * If it is provided, it MUST resolve to a NUMBER. */
nifti_operator
    : 'nifti' '(' ( selector ',' )? bag_expression ( ',' STRING )? ')'
    ;

json_operator
    : 'json' '(' jslt ',' bag_expression ( ',' STRING )? ')'
    ;

jslt
    : json
    ;

/**********************************************************************/
/* JSON                                                               */
/**********************************************************************/

/**
 * Taken and adapted from:
 *  https://github.com/antlr/grammars-v4/blob/master/json/JSON.g4
 *
 * Some of the parser / lexer rules are in the imported grammar as well.
 */
json
    : json_value
    ;

json_obj
    : '{' json_pair (',' json_pair)* '}'
    | '{' '}'
    ;

json_pair
    : STRING ':' json_value
    ;

json_array
    : '[' json_value (',' json_value)* ']'
    | '[' ']'
    ;

json_value
    : STRING
    | json_number
    | json_obj
    | json_array
    | 'true'
    | 'false'
    | 'null'
    /* Add support to reference values from the selected bag. */
    | selector
    | aggregation_expr
    ;

/* The bag expression is implicit here, as this is te
 * second argument to the json operator */
aggregation_expr
    : 'count' '(' 'distinct'? selector ')'
    | 'sum' '(' selector ')'
    | 'min' '(' selector ')'
    | 'max' '(' selector ')'
    | 'nifti' '(' selector ')'
    | 'mbb' '(' ')'
    ;
