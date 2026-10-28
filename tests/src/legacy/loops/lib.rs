//! @fail(tc): legacy-lean(1)
//! @fail(tc): fstar(72)

mod recognized_loops {
/// @fail(extraction): proverif(HAX0008)
    fn range() {
        let mut count = 0u64;
        for i in 0u8..10u8 {
            hax_lib::loop_invariant!(|i: u8| i <= 10);
            count += 1;
        }
    }
/// @fail(extraction): proverif(HAX0008)
    fn range_step_by() {
        let mut count = 0u64;
        for i in (0u8..10u8).step_by(2) {
            hax_lib::loop_invariant!(|i: u8| i <= 10);
            count += 1;
        }
    }
/// @fail(extraction): proverif(HAX0008)
    fn enumerated_slice<T>(slice: &[T]) {
        let mut count = 0u64;
        for i in slice.into_iter().enumerate() {
            hax_lib::loop_invariant!(|i: usize| i <= 10);
            count += 2;
        }
    }
/// @fail(extraction): proverif(HAX0008)
    fn enumerated_chunked_slice<T>(slice: &[T]) {
        let mut count = 0u64;
        for i in slice.chunks_exact(3).enumerate() {
            hax_lib::loop_invariant!(|i: usize| { fstar!("$i <= ${slice.len()}") });
            count += 3;
        }
    }
}

mod for_loops {
/// @fail(extraction): proverif(HAX0008)
    fn range1() -> usize {
        let mut acc = 0;
        for i in 0..15 {
            acc = acc + i;
        }
        acc
    }

/// @fail(extraction): proverif(HAX0008)
    fn range2(n: usize) -> usize {
        let mut acc = 0;
        for i in 0..(n + 10) {
            acc = acc + i + 1;
        }
        acc
    }

/// @fail(extraction): proverif(HAX0008)
    fn composed_range(n: usize) -> usize {
        let mut acc = 0;
        for i in (0..n).chain((n + 10)..(n + 50)) {
            acc = acc + i + 1;
        }
        acc
    }

/// @fail(extraction): proverif(HAX0008)
    fn rev_range(n: usize) -> usize {
        let mut acc = 0;
        for i in (0..n).rev() {
            acc = acc + i + 1;
        }
        acc
    }

/// @fail(extraction): proverif(HAX0008, HAX0008)
    fn chunks<const CHUNK_LEN: usize>(arr: Vec<usize>) -> usize {
        let mut acc = 0;
        let chunks = arr.chunks_exact(CHUNK_LEN);
        for chunk in chunks.clone() {
            let mut mean = 0;
            for item in chunk {
                mean = mean + item;
            }
            acc = acc + mean / CHUNK_LEN;
        }
        for item in chunks.remainder() {
            acc = acc - item;
        }
        acc
    }

/// @fail(extraction): proverif(HAX0008)
    fn iterator(arr: Vec<usize>) -> usize {
        let mut acc = 0;
        for item in arr.iter() {
            acc = acc + item;
        }
        acc
    }

/// @fail(extraction): ssprove(HAX0001), proverif(HAX0008)
    fn nested(arr: Vec<usize>) -> usize {
        let mut acc = 0;
        for item in arr.iter() {
            for i in (0..*item).rev() {
                acc = acc + 1;
                for j in arr.iter().zip(4..i) {
                    acc = acc + item + i + j.0 + j.1;
                }
            }
        }
        acc
    }

/// @fail(extraction): proverif(HAX0008)
    fn pattern(arr: Vec<(usize, usize)>) -> usize {
        let mut acc = 0;
        for (x, y) in arr {
            acc = acc + x * y;
        }
        acc
    }

/// @fail(extraction): proverif(HAX0008)
    fn enumerate_chunks(arr: Vec<usize>) -> usize {
        let mut acc = 0;
        for (i, chunk) in arr.chunks(4).enumerate() {
            for (j, x) in chunk.iter().enumerate() {
                acc = i + j + x;
            }
        }
        acc
    }

    fn bool_returning(x: u8) -> bool {
        x < 10
    }

/// @fail(extraction): proverif(HAX0008)
    fn f() {
        let mut acc = 0;
        for i in 1..10 {
            acc += i;
            bool_returning(i);
        }
    }
}

mod while_loops {
/// @fail(extraction): coq(HAX0001, HAX0001), ssprove(HAX0001), proverif(HAX0008)
    fn f() -> u8 {
        let mut x = 0;
        while x < 10 {
            x = x + 3;
        }
        x + 12
    }
/// @fail(extraction): proverif(HAX0008), ssprove(HAX0001), coq(HAX0001, HAX0001)
    fn while_invariant_decr() -> u8 {
        let mut x = 0;
        while x < 10 {
            hax_lib::loop_invariant!(x <= 10);
            hax_lib::loop_decreases!(10 - x);
            x = x + 3;
        }
        x + 12
    }
/// @fail(extraction): ssprove(HAX0001), coq(HAX0001, HAX0001), proverif(HAX0008)
    fn while_invariant_decr_rev() -> u8 {
        let mut x = 0;
        while x < 10 {
            hax_lib::loop_decreases!(10 - x);
            hax_lib::loop_invariant!(x <= 10);
            x = x + 3;
        }
        x + 12
    }
}

mod control_flow {
/// @fail(extraction): coq(HAX0001), ssprove(HAX0001), proverif(HAX0008)
    fn double_sum() -> i32 {
        let mut sum = 0;
        for i in 1..10 {
            if i < 0 {
                break;
            }
            sum += i;
        }
        sum *= 2;
        sum
    }
/// @fail(extraction): coq(HAX0001), ssprove(HAX0001), proverif(HAX0008)
    fn double_sum2() -> i32 {
        let mut sum = 0;
        let mut sum2 = 0;
        for i in 1..10 {
            if i < 0 {
                break;
            }
            sum += i;
            sum2 += i
        }
        sum + sum2
    }
/// @fail(extraction): proverif(HAX0008)
    fn double_sum_return(v: &[i32]) -> i32 {
        let mut sum = 0;
        for i in v {
            if *i < 0 {
                return 0;
            }
            sum += *i;
        }
        sum *= 2;
        sum
    }
/// @fail(extraction): proverif(HAX0008)
    fn double_sum2_return(v: &[i32]) -> i32 {
        let mut sum = 0;
        let mut sum2 = 0;
        for i in v {
            if *i < 0 {
                return 0;
            }
            sum += *i;
            sum2 += *i
        }
        sum + sum2
    }
/// @fail(extraction): coq(HAX0001, HAX0001, HAX0001), proverif(HAX0008), ssprove(HAX0001, HAX0001)
    fn bigger_power_2(x: i32) -> i32 {
        let mut pow = 1;
        while pow < 1000000 {
            pow *= 2;
            if pow < x {
                pow *= 3;
                if true {
                    break;
                }
            }
            pow *= 2
        }
        pow
    }
    struct M {
        m: Vec<u8>,
    }

/// @fail(extraction): proverif(HAX0008)
    impl M {
        fn decoded_message(&self) -> Option<Vec<u8>> {
            for i in 0..self.m.len() {
                if i > 5 {
                    return None;
                }
            }
            return Some(self.m.clone());
        }
    }
/// @fail(extraction): proverif(HAX0008), ssprove(HAX0001), coq(HAX0001)
    fn nested() -> i32 {
        let mut sum = 0;
        for i in 1..10 {
            for j in 1..10 {
                if j < 0 {
                    break;
                }
                sum += j;
            }
            sum += i;
        }
        sum *= 2;
        sum
    }
/// @fail(extraction): proverif(HAX0008)
    fn nested_return() -> i32 {
        let mut sum = 0;
        for i in 1..10 {
            for j in 1..10 {
                if j < 0 {
                    return 0;
                }
                sum += j;
            }
            sum += i;
        }
        sum *= 2;
        sum
    }
/// @fail(extraction): ssprove(HAX0008), coq(HAX0008), proverif(HAX0008, HAX0008)
    fn continue_only(x: &[i32]) {
        let mut product = 1;
        for i in x {
            if *i == 0 {
                continue;
            }
            product *= i
        }
    }
/// @fail(extraction): proverif(HAX0008, HAX0008), ssprove(HAX0001, HAX0008), coq(HAX0008, HAX0001)
    fn continue_and_break(x: &[i32]) {
        let mut product = 1;
        for i in x {
            if *i == 0 {
                continue;
            }
            if *i < 0 {
                break;
            }
            product *= i
        }
    }
}

mod and_mut_side_effect_loop {
    // https://github.com/pereq-dev/hax/issues/720
/// @fail(extraction): proverif(HAX0008)
    fn looping(array: &mut [u8; 5]) {
        for i in 0..array.len() {
            array[i] = i as u8;
        }
    }

    #[hax_lib::fstar::verification_status(panic_free)]
/// @fail(extraction): proverif(HAX0008)
    fn looping_2(array: &mut [u8; 5]) {
        for i in 0..array.len() {
            array[i] = i as u8;
        }
    }
}
