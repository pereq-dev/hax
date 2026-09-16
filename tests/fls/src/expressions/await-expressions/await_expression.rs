//! @fls: fls_1ppywe40s62c, fls_c6mxfzef2qop, fls_eymcs2rgv3qw, fls_frwtufwe8dya, fls_k76d8ady623m, fls_l396mo6k9ox7, fls_qshnnpirkasz, fls_sjz5s71hwm7l, fls_tn3vwidct3ks, fls_u72ylhlmqge3, fls_umevprl99y6c, fls_vhchgab59jvd
//! @title: An await expression inside an async function
//! @lean(tier): unsupported
//! @lean(reason): The value-model boundary of the input-language reference: async compiles to a poll-driven state machine whose suspension points need an execution model.

pub async fn one() -> u32 {
    1
}

pub async fn two() -> u32 {
    one().await + one().await
}

pub fn keep() -> u32 {
    let _fut = two();
    2
}

#[hax_test::equiv]
pub fn future_constructed() -> bool {
    keep() == 2
}
