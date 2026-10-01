//! @off: fstar, lean, coq, ssprove, legacy-lean
//!
//! A match destructures its scrutinee against the `[data]` constructors of the
//! matched type. A match on an uninhabited type has no arms and renders as the
//! error sink.

enum Direction {
    North,
    South,
}

fn opposite(d: Direction) -> Direction {
    match d {
        Direction::North => Direction::South,
        Direction::South => Direction::North,
    }
}

fn or_zero(x: Option<u32>) -> u32 {
    match x {
        Some(v) => v,
        None => 0,
    }
}

enum Void {}

fn absurd(v: Void) -> u32 {
    match v {}
}
