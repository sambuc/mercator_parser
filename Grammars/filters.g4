grammar filters;

/**********************************************************************/
/* SELECTING / FILTERING DATA                                         */
/**********************************************************************/
filters
    : bag_expression
    ;

/* All these expressions generate bags. */
bag_expression
    // Bag Operators
    : distinct
    | filter
    | complement
    | intersection
    | union
    | bag
    // Spatial Operators
    | inside
    | outside
    ;

/**********************************************************************/
/* BAG OPERATORS                                                      */
/**********************************************************************/
distinct
    : 'distinct' '(' bag_expression ')'
    ;

/* Returns all the points which are NOT part of the bag. */
complement
    : 'complement' '(' bag_expression ')'
    ;

/* Returns points which are part of both left and right sets. */
intersection
    : 'intersection' '(' bag_expression ',' bag_expression ')'
    ;

/* Returns points which are either part of left or right sets
 * (or both). */
union
    : 'union' '(' bag_expression ',' bag_expression ')'
    ;

/* Filters point so that points part of the resulting bag respect
 * the predicate. */
filter
    : 'filter' '(' ( bag_expression | predicate ( ',' bag_expression )? ) ')'
    ;

predicate
    : less
    | greater
    | equal
    | str_cmp
    | not
    | and
    | or
    ;

less
    : '<' '(' position_expr ',' position ')'
    ;

greater
    : '>' '(' position_expr ',' position ')'
    ;

equal
    : '=' '(' position_expr ',' position ')'
    ;

not
    : '!' '(' predicate ')'
    ;

and
    : '&' '(' predicate ',' predicate ')'
    ;

or
    : '|' '(' predicate ',' predicate ')'
    ;

/* Arbitrary bag of positions. */
bag
    : 'bag' '{' bag_expression (',' bag_expression )* '}'
    ;

/**********************************************************************/
/* SPATIAL OPERATORS                                                  */
/**********************************************************************/

/* Faces | vertices are included to allow selection on a pure plane or
 * boundary.
 *
 * For example:
 *   intersection(outside(hyperrectangle{[0,0], [1,1]},
 *                inside(hyperrectangle{[0,0], [1,1]})
 * will be true for any point lying EXACTLY on a face, corner or edge
 * of the cube [0,0], [1,1].
 */

/* Returns the set of points outside the shape, (face included) */
outside
    : 'outside' '(' shapes ')'
    ;

/* Returns the set of points inside the shape, (face included) */
inside
    : 'inside' '(' shapes ')'
    ;

/**********************************************************************/
/* SHAPES                                                             */
/**********************************************************************/
shapes
    : point
    | hyperrectangle
    | hypersphere
    | nifti
    ;

/* If the hyperrectangle is aligned with the axes, then two points are
 * enough, if not we need all the points to be specified.
 */
hyperrectangle
    : 'hyperrectangle' '{'
          position ',' position
          ( ',' position ',' position )*
          ( ',' STRING )?
       '}'
    ;

/* A hypersphere is defined by its center and a radius, independantly
 * of the number of dimensions of the space. */
hypersphere
    : 'hypersphere' '{'
           position
           ',' positive_number
           ( ',' STRING )?
        '}'
    ;

point
    : 'point' '{' position ( ',' STRING )? '}'
    ;

/* Define a shape as the non-zero values in a NIfTI object, defined by
 *   nifti{
 *     lower_corner: position,  // Optional, default to the origin
 *     rotation: [ position+ ], // Optional, no rotation by default
 *     bytes: uri(STRING),      // uri to the NIfTI object
 *     spaceId: string
 *   }
 */
nifti
    : 'nifti' '{'
        (position ',' )?
        ( '[' position ( ',' position )* ']' ',' )?
        byte_provider ','
        STRING
      '}'
    ;

/* TODO: STRING is assumed to be a well-formed URI, fully specify here?
 *
 * TODO: Add a provider for in-line raw-byte stream.
 */
byte_provider
    : 'uri' '(' STRING ')'
    ;

/**********************************************************************/
/* POSITIONS                                                          */
/**********************************************************************/

/* Always returns a vector of numbers, a.k.a a position (a scalar will
 * be represented as a vector of one element) */
position_expr
    : str_cmp_icase
    | str_cmp
    | selector
    | position
    ;

/* Compare lexicographically two strings, and returns a `position`:
 *  [-1] : String is lexicographically before,
 *  [ 0] : is equal,
 *  [ 1] : is after.
 */
str_cmp
    : 'str_cmp' '(' selector ',' STRING ')'
    ;

/* Same, but case insensitive. */
str_cmp_icase
    : 'str_cmp_ignore_case' '(' selector ',' STRING ')'
    ;

/* TODO: FIELDS are expected to be exisiting in the data model. Root Object is assumed to be the type of the ressource on which the POST call was done.
 */
selector
    : ( FIELD )+
    ;

position
    : '[' number ( ',' number )* ']'
    ;

/**********************************************************************/
/* TOKENS - STRINGS                                                   */
/**********************************************************************/

/* Accept field descriptor which
 *  1. start with a dot ('.')
 *  2. optionnally followed by a field name consisting of a letter or
 *     underscore, followed by letters, numbers or underscore,
 *  3. optionnally followed by brakets enclosing an natural number
 *     denoting an offset in a list or array. */
FIELD
    : '.' ( [a-zA-Z_] [a-zA-Z0-9_]* )? ('[' INTEGER ']')?
    ;

STRING
   : '"' (ESC | SAFECODEPOINT)* '"'
   ;

fragment ESC
   : '\\' (["\\/bfnrt] | UNICODE)
   ;

fragment UNICODE
   : 'u' HEX HEX HEX HEX
   ;

fragment HEX
   : [0-9a-fA-F]
   ;

fragment SAFECODEPOINT
   : ~ ["\\\u0000-\u001F]
   ;

/**********************************************************************/
/* TOKENS - NUMBERS                                                   */
/**********************************************************************/
/* We define 3 kinds of number, to avoid ambiguities in the rules. */

/* No optional leading '+' */
json_number
    : '-'? NUM
    ;

positive_number
    : '+'? NUM
    ;

number
    : ( '+' | '-' )? NUM
    ;

NUM
    :  INTEGER ('.' [0-9]+ )? EXP?
    ;

fragment EXP
    : [Ee] [+\-]? INTEGER
    ;


/* No leading zeros */
fragment INTEGER
    : '0' | [1-9] [0-9]*
    ;

/**********************************************************************/
/* WHITESPACES & COMMENTS                                             */
/**********************************************************************/
COMMENTS
    : ( '//' ~[\r\n]* | '/*' .*? '*/' ) -> skip
    ;

WS
    : [ \t\r\n]+ -> skip
    ; // skip spaces, tabs, newlines
