// Regression test for #135235.
trait Foo {
    type Assoc;

    fn from(s: Self::Assoc) -> Self;
}

struct W<T>(T);

/// @fail(extraction): fstar(HAX0001), coq(HAX0001), ssprove(HAX0001), legacy-lean(HAX0001)
/// @fail(extraction): proverif(HAX0001)
impl<T: Foo> From<[T::Assoc; 1]> for W<T> {
    fn from(from: [T::Assoc; 1]) -> Self {
        let [item] = from;
        W(Foo::from(item))
    }
}

fn main() {}
