#[derive(Copy, Clone)]
pub enum Op {
    Push(i64),
    Add,
    Sub,
    Equal,
    LessThan,
    GreaterThan,
    LessThanEqual,
    GreaterThanEqual,
    If(usize),
    Else(usize),
    End,
    Duplicate,
    Dump,
}
