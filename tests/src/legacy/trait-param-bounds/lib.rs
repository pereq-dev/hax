//! Bounds on the generic parameters of a trait are parent bounds of its impls,
//! just like its supertraits (https://github.com/pereq-dev/hax/issues/1554).
//! @off: coq, ssprove, proverif

#![allow(dead_code)]

trait Item {
    fn get(&self) -> u8;
}
trait Other {}

impl Item for u64 {
    fn get(&self) -> u8 {
        0
    }
}
impl Item for u16 {
    fn get(&self) -> u8 {
        1
    }
}
impl Other for u32 {}

/// Bound on a parameter.
trait Squeeze<T: Item> {
    fn squeeze(&self, x: &T) -> u8;
}
impl Squeeze<u64> for u8 {
    fn squeeze(&self, x: &u64) -> u8 {
        x.get()
    }
}

/// Generic impl.
struct W;
impl<T: Item> Squeeze<T> for W {
    fn squeeze(&self, x: &T) -> u8 {
        x.get()
    }
}

/// Supertrait mixed with parameter bounds, inline and in a `where` clause.
trait Mixed<A: Item, B>: Other
where
    B: Item + Clone,
{
    fn mixed(&self, a: &A, b: &B) -> bool;
}
impl Mixed<u64, u16> for u32 {
    fn mixed(&self, a: &u64, b: &u16) -> bool {
        a.get() == b.get()
    }
}

/// Supertrait in a `where` clause, after a parameter bound.
trait WhereSuper<A: Item>
where
    Self: Other,
{
    fn where_super(&self, a: &A) -> u8;
}
impl WhereSuper<u64> for u32 {
    fn where_super(&self, a: &u64) -> u8 {
        a.get()
    }
}

/// Bound on an associated type of a supertrait.
trait HasAssoc {
    type X;
}
trait AssocBound: HasAssoc
where
    Self::X: Clone,
{
    fn assoc_bound(&self) -> u8;
}
impl HasAssoc for u32 {
    type X = u64;
}
impl AssocBound for u32 {
    fn assoc_bound(&self) -> u8 {
        2
    }
}

/// Trait without items.
trait Marker<T: Item> {}
impl Marker<u64> for u8 {}

fn use_mixed<S: Mixed<A, B>, A: Item, B: Item + Clone>(s: &S, a: &A, b: &B) -> bool {
    s.mixed(a, b)
}
fn call_use_mixed() -> bool {
    use_mixed(&0u32, &0u64, &0u16)
}
