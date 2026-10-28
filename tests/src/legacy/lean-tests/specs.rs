#[hax_lib::requires(x > 0)]
#[hax_lib::ensures(|r| r == x)]
fn test(x: u8) -> u8 {
    x
}

#[hax_lib::requires(x > 0)]
#[hax_lib::ensures(|r| r == x)]
fn use_previous_result(x: u8) -> u8 {
    test(x)
}

#[hax_lib::requires(x > 0)]
#[hax_lib::ensures(|r| r == x)]
#[hax_lib::legacy_lean::proof("by unfold lean_tests.specs.test_proof; hax_bv_decide")]
fn test_proof(x: u8) -> u8 {
    x
}

#[hax_lib::requires(x < 16)]
#[hax_lib::ensures(|res| res >= x)]
fn square(x: u8) -> u8 {
    x * x
}

#[hax_lib::requires(hax_lib::forall(|i:u8| hax_lib::implies(i < 20, x > i)))]
#[hax_lib::ensures(|r| !hax_lib::exists(|i:u8| !hax_lib::implies(i < 20, r > i)))]
#[hax_lib::legacy_lean::proof_method::grind]
fn forall_and_exists(x: u8) -> u8 {
    x
}

/// Test function without arguments
/// https://github.com/pereq-dev/hax/issues/1856
#[hax_lib::ensures(|_| true)]
fn fn_without_args() {}

/// The Lean backend used to produce `self_` instead of `self` in annotations in
/// impl blocks. See https://github.com/pereq-dev/hax/issues/1852.
mod issue_1852 {
    struct T {}

    #[hax_lib::attributes]
    impl T {
        pub fn test(self) -> bool {
            true
        }

        #[hax_lib::requires(T::test(self))]
        pub fn func(self) {}
    }
}

#[hax_lib::requires(true)]
#[hax_lib::ensures(|r| true)]
#[hax_lib::legacy_lean::pure_requires_proof("⟨True, by mvcgen⟩")]
#[hax_lib::legacy_lean::pure_ensures_proof("⟨fun _ => True, by intros; mvcgen⟩")]
fn custom_pure_proofs(x: u8) {}

/// Resugarings need to be apply also to linked items
/// https://github.com/pereq-dev/hax/issues/1945
mod issue_1945 {
    #[hax_lib::requires({let x = a; a == 0})]
    fn mktuple(a: i32) -> bool {
        {let x = a; a == 0}
    }
}
