#[derive(Debug,PartialEq)]
pub enum Keyword {
    Select,  // SELECT
    From,    // FROM
    Where,   // WHERE
    And,     // AND
    Or,      // OR
    Update,  // UPDATE
    Set,     // SET
    Insert,  // INSERT
    Into,    // INTO
    Values,  // VALUES
    Inner,   // INNER
    Join,    // JOIN
    On,      // ON
    Limit,   // LIMIT
    Offset,  // OFFSET
    Between, // BETWEEN
    Array,   // ARRAY
}