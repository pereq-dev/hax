//! @fls: hax_future, hax_ensures
//! @title: A postcondition naming the post-state of a mutable argument with future
//! @lean(tier): supported

#[hax_lib::ensures(|_| *future(counter) == *counter + 1)]
pub fn increment(counter: &mut u32) {
    *counter += 1;
}

#[hax_test::equiv]
pub fn post_state_matches() -> bool {
    let mut c = 41;
    increment(&mut c);
    c == 42
}
