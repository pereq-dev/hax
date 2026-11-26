/// See [`std::borrow::Borrow`]
trait Borrow<Borrowed: ?Sized> {
    /// See [`std::borrow::Borrow::borrow`]
    fn borrow(&self) -> &Borrowed;
}

/// See [`std::borrow::Borrow`] for `&T`
impl<T: ?Sized> Borrow<T> for &T {
    fn borrow(&self) -> &T {
        &**self
    }
}

#[cfg(test)]
mod tests {
    use super::Borrow;

    #[test]
    fn test_borrow_shared() {
        let x: &[u8] = &[1, 2, 3];
        assert_eq!(
            Borrow::<[u8]>::borrow(&x),
            std::borrow::Borrow::<[u8]>::borrow(&x)
        );
    }
}
