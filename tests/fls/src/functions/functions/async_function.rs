//! @fls: fls_m3jiunibqj81, fls_7vogmqyd87ey, fls_5Q861wb08DU3
//! @title: An async function returning a future
//! @lean(tier): unsupported
//! @lean(reason): The value-model boundary of the input-language reference: async compiles to a poll-driven state machine whose suspension points need an execution model.

pub async fn compute(x: u32) -> u32 {
    x + 1
}

pub fn build() -> u32 {
    let _future = compute(1);
    2
}

#[hax_test::equiv]
pub fn future_created() -> bool {
    build() == 2
}
