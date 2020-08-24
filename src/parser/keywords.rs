#[derive(Debug)]
pub enum Keyword {
    Insert,
    Select,
    Update,
    Delete,

    From,
    Where,

    With,
    Recursive,

    GroupBy,

    Having,

    Window,
    As,

    Union,
    Intersect,
    Except,
    All,
    Distinct,

    OrderBy,

    Limit,

    Offset,
    Row,
    Rows,
    Fetch,
    First,
    Next,
    Only,

    For,
    NoKeyUpdate,
}
