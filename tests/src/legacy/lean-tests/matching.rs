/// @fail(extraction): ssprove(HAX0001)
fn test_const_matching(x: u32, c: char, s: &str, b: bool) -> u32 {
    let x = match x {
        0 => 42,
        _ => 0,
    };
    let c = match c {
        'a' => 42,
        _ => 0,
    };
    let s = match s {
        "Hello" => 42,
        _ => 0,
    };
    let b = match b {
        true => 42,
        false => 0,
    };
    return x + c + s + b;
}

/// @fail(extraction): fstar(HAX0008), ssprove(HAX0008), coq(HAX0008)
fn test_binding_subpattern_matching(x: (u8, (u8, u8))) -> u8 {
    match x {
        (0, pair @ (a, b)) => a + b + pair.0 + pair.1,
        _ => 0,
    }
}

fn test_ellipsis_records() {
    enum E {
        C { f1: u8, f2: u8, f3: u8, f4: u8 },
    }

    let c = E::C {
        f1: 1,
        f2: 2,
        f3: 3,
        f4: 4,
    };

    match c {
        E::C { .. } => assert!(true),
    };
    match c {
        E::C { f1, .. } => assert!(f1 == 1),
    };
    match c {
        E::C { f1, f2, .. } => assert!(f1 == 1 && f2 == 2),
    };
    match c {
        E::C { f2, f4, .. } => assert!(f2 == 2 && f4 == 4),
    };
    match c {
        E::C { f1, f2, f3, f4 } => assert!(f1 == 1 && f2 == 2 && f3 == 3 && f4 == 4),
    };
}

fn test_ellipsis_structs() {
    struct S {
        f1: u8,
        f2: u8,
        f3: u8,
        f4: u8,
    }

    let c = S {
        f1: 1,
        f2: 2,
        f3: 3,
        f4: 4,
    };

    match c {
        S { .. } => assert!(true),
    };
    match c {
        S { f1, .. } => assert!(f1 == 1),
    };
    match c {
        S { f1, f2, .. } => assert!(f1 == 1 && f2 == 2),
    };
    match c {
        S { f2, f4, .. } => assert!(f2 == 2 && f4 == 4),
    };
    match c {
        S { f1, f2, f3, f4 } => assert!(f1 == 1 && f2 == 2 && f3 == 3 && f4 == 4),
    };
}

fn test_ellipsis_bare_tuples() {
    let t = (1u8, 2u8, 3u8, 4u8);

    match t {
        (..) => assert!(true),
    };
    match t {
        (a, ..) => assert!(a == 1),
    };
    match t {
        (a, b, ..) => assert!(a == 1 && b == 2),
    };
    match t {
        (.., d) => assert!(d == 4),
    };
    match t {
        (.., c, d) => assert!(c == 3 && d == 4),
    };
    match t {
        (a, .., d) => assert!(a == 1 && d == 4),
    };
    match t {
        (a, b, c, d) => assert!(a == 1 && b == 2 && c == 3 && d == 4),
    };
}

fn test_ellipsis_tuples() {
    enum F {
        D(u8, u8, u8, u8),
    }

    let d = F::D(1, 2, 3, 4);

    match d {
        F::D(..) => assert!(true),
    };
    match d {
        F::D(a, ..) => assert!(a == 1),
    };
    match d {
        F::D(a, b, ..) => assert!(a == 1 && b == 2),
    };
    match d {
        F::D(.., d) => assert!(d == 4),
    };
    match d {
        F::D(.., c, d) => assert!(c == 3 && d == 4),
    };
    match d {
        F::D(a, .., d) => assert!(a == 1 && d == 4),
    };
    match d {
        F::D(a, b, c, d) => assert!(a == 1 && b == 2 && c == 3 && d == 4),
    };
}
