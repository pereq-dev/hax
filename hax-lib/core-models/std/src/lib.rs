#![allow(unused)]
// `coverage(off)` is unstable; `cfg(coverage_nightly)` is set only by
// `cargo llvm-cov`, so normal builds and extraction never see this.
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

mod collections {
    mod hash {
        mod map {
            #[hax_lib::opaque]
            struct HashMap<K, V, S>(Option<K>, Option<V>, Option<S>);
            impl<K, V> HashMap<K, V, crate::hash::random::RandomState> {
                #[hax_lib::opaque]
                fn new() -> HashMap<K, V, crate::hash::random::RandomState> {
                    HashMap(None, None, None)
                }
            }
            // Dummy impl for disambiguator (https://github.com/pereq-dev/hax/issues/828)
            impl HashMap<usize, usize, usize> {}
            impl<K, V, S> HashMap<K, V, S> {
                // Excluded from coverage: `HashMap` is `hax_lib::opaque`, so it
                // carries no representation to hold entries in, and there is no
                // lookup for a body to perform.
                #[cfg_attr(coverage_nightly, coverage(off))]
                #[hax_lib::opaque]
                fn get<Y>(m: HashMap<K, V, S>, k: K) -> core_models::option::Option<V> {
                    core_models::panicking::internal::panic()
                }
                // Excluded from coverage: see `get`.
                #[cfg_attr(coverage_nightly, coverage(off))]
                #[hax_lib::opaque]
                fn insert(
                    m: HashMap<K, V, S>,
                    k: K,
                    v: V,
                ) -> (HashMap<K, V, S>, core_models::option::Option<V>) {
                    core_models::panicking::internal::panic()
                }
            }

            #[cfg(test)]
            mod tests {
                #[test]
                fn test_new_is_empty() {
                    // `new` is the only runnable item here: the type is an
                    // interface stub with no entries to compare against std's.
                    let m = super::HashMap::<u8, u8, crate::hash::random::RandomState>::new();
                    assert!(m.0.is_none());
                    assert!(m.1.is_none());
                    assert!(m.2.is_none());
                }
            }
        }
    }
}

mod f64 {
    #[hax_lib::exclude]
    #[allow(non_camel_case_types)]
    struct f64;
    impl f64 {
        fn powf(x: core::primitive::f64, y: core::primitive::f64) -> core::primitive::f64 {
            rust_primitives::float::powf_f64(x, y)
        }
    }

    #[cfg(test)]
    mod tests {
        use proptest::prelude::*;

        proptest! {
            // Compared on bits so that NaN results still count.
            #[test]
            fn test_powf(x in any::<f64>(), y in any::<f64>()) {
                prop_assert_eq!(super::f64::powf(x, y).to_bits(), x.powf(y).to_bits());
            }
        }
    }
}

pub mod hash {
    pub mod random {
        pub struct RandomState;
    }
}

mod io {
    #[hax_lib::attributes]
    pub trait Read {
        // Required method
        #[hax_lib::requires(true)]
        #[hax_lib::ensures(|_| future(buf).len() == buf.len())]
        fn read(&mut self, buf: &mut [u8]) -> Result<usize, error::Error>;

        // Provided methods (not provided in this model as hax doesn't support default methods)
        /* fn read_vectored(&mut self, bufs: &mut [IoSliceMut<'_>]) -> Result<usize>;
        fn is_read_vectored(&self) -> bool;
        fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize>;
        fn read_to_string(&mut self, buf: &mut String) -> Result<usize>; */
        #[hax_lib::requires(true)]
        #[hax_lib::ensures(|_| future(buf).len() == buf.len())]
        fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), error::Error>;
        /* fn read_buf(&mut self, buf: BorrowedCursor<'_>) -> Result<()>;
        fn read_buf_exact(&mut self, cursor: BorrowedCursor<'_>) -> Result<()>;
        fn by_ref(&mut self) -> &mut Self
        where Self: Sized;
        fn bytes(self) -> Bytes<Self>
        where Self: Sized;
        fn chain<R: Read>(self, next: R) -> Chain<Self, R>
        where Self: Sized;
        fn take(self, limit: u64) -> Take<Self>
        where Self: Sized; */
    }
    #[hax_lib::attributes]
    pub trait Write {
        // Required methods
        #[hax_lib::requires(true)]
        fn write(&mut self, buf: &[u8]) -> Result<usize, error::Error>;
        #[hax_lib::requires(true)]
        fn flush(&mut self) -> Result<(), error::Error>;

        // Provided methods (not provided in this model as hax doesn't support default methods)
        /* fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> Result<usize>;
        fn is_write_vectored(&self) -> bool; */
        #[hax_lib::requires(true)]
        fn write_all(&mut self, buf: &[u8]) -> Result<(), error::Error>;
        /* fn write_all_vectored(&mut self, bufs: &mut [IoSlice<'_>]) -> Result<()>;
        fn write_fmt(&mut self, args: Arguments<'_>) -> Result<()>;
        fn by_ref(&mut self) -> &mut Self
        where Self: Sized; */
    }
    pub mod error {
        pub struct Error;
        pub enum ErrorKind {
            NotFound,
            PermissionDenied,
            ConnectionRefused,
            ConnectionReset,
            HostUnreachable,
            NetworkUnreachable,
            ConnectionAborted,
            NotConnected,
            AddrInUse,
            AddrNotAvailable,
            NetworkDown,
            BrokenPipe,
            AlreadyExists,
            WouldBlock,
            NotADirectory,
            IsADirectory,
            DirectoryNotEmpty,
            ReadOnlyFilesystem,
            FilesystemLoop,
            StaleNetworkFileHandle,
            InvalidInput,
            InvalidData,
            TimedOut,
            WriteZero,
            StorageFull,
            NotSeekable,
            QuotaExceeded,
            FileTooLarge,
            ResourceBusy,
            ExecutableFileBusy,
            Deadlock,
            CrossesDevices,
            TooManyLinks,
            InvalidFilename,
            ArgumentListTooLong,
            Interrupted,
            Unsupported,
            UnexpectedEof,
            OutOfMemory,
            InProgress,
            Other,
        }
        impl Error {
            // Excluded from coverage: the model's `Error` is a unit struct, so it
            // records no kind for a body to report.
            #[cfg_attr(coverage_nightly, coverage(off))]
            #[hax_lib::opaque]
            fn kind(&self) -> ErrorKind {
                core_models::panicking::internal::panic()
            }
        }
    }
    mod impls {
        impl super::Read for &[u8] {
            fn read(&mut self, buf: &mut [u8]) -> Result<usize, super::error::Error> {
                let amt = core::cmp::min(buf.len(), self.len());
                let (a, b) = self.split_at(amt);

                buf[..amt].copy_from_slice(a);

                *self = b;
                Ok(amt)
            }
            fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), super::error::Error> {
                if buf.len() > self.len() {
                    // `read_exact` makes no promise about the content of `buf` if it
                    // fails so don't bother about that.
                    *self = &self[self.len()..];
                    return Err(super::error::Error);
                }
                let (a, b) = self.split_at(buf.len());

                buf.copy_from_slice(a);

                *self = b;
                Ok(())
            }
        }
        impl super::Write for Vec<u8> {
            fn write(&mut self, buf: &[u8]) -> Result<usize, super::error::Error> {
                self.extend_from_slice(buf);
                Ok(buf.len())
            }
            fn write_all(&mut self, buf: &[u8]) -> Result<(), super::error::Error> {
                self.extend_from_slice(buf);
                Ok(())
            }
            fn flush(&mut self) -> Result<(), super::error::Error> {
                Ok(())
            }
        }
    }
    mod stdio {
        #[hax_lib::opaque]
        fn e_print(args: core::fmt::Arguments) {}

        /// See [`std::io::stdio::_print`], what `print!`/`println!` expand to.
        /// Like `e_print` above, the model prints nothing.
        //
        // Excluded on the F* lane: hax renders a leading underscore as `e_`, so
        // `_print` and `e_print` would collide. `std` is extracted to F* only,
        // so this item has no backend counterpart — it exists for the clients
        // that mention it and for the coverage count.
        #[cfg_attr(hax_backend_fstar, hax_lib::exclude)]
        #[hax_lib::opaque]
        fn _print(args: core::fmt::Arguments) {}

        #[cfg(test)]
        mod tests {
            use proptest::prelude::*;

            proptest! {
                // The model prints nothing; all there is to check is that it runs.
                #[test]
                fn test_print(x in any::<u8>()) {
                    super::_print(format_args!("{x}"));
                }

                #[test]
                fn test_e_print(x in any::<u8>()) {
                    super::e_print(format_args!("{x}"));
                }
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::{Read, Write};
        use proptest::prelude::*;

        proptest! {
            #[test]
            fn test_read(
                src in prop::collection::vec(any::<u8>(), 0..50),
                buf_len in 0usize..50,
            ) {
                let mut model_src: &[u8] = &src;
                let mut model_buf = vec![0u8; buf_len];
                let model_n = Read::read(&mut model_src, &mut model_buf);

                let mut std_src: &[u8] = &src;
                let mut std_buf = vec![0u8; buf_len];
                let std_n = std::io::Read::read(&mut std_src, &mut std_buf);

                prop_assert_eq!(model_n.ok(), std_n.ok());
                prop_assert_eq!(model_buf, std_buf);
                prop_assert_eq!(model_src, std_src);
            }

            #[test]
            fn test_read_exact(
                src in prop::collection::vec(any::<u8>(), 0..50),
                buf_len in 0usize..50,
            ) {
                let mut model_src: &[u8] = &src;
                let mut model_buf = vec![0u8; buf_len];
                let model_res = Read::read_exact(&mut model_src, &mut model_buf);

                let mut std_src: &[u8] = &src;
                let mut std_buf = vec![0u8; buf_len];
                let std_res = std::io::Read::read_exact(&mut std_src, &mut std_buf);

                prop_assert_eq!(model_res.is_err(), std_res.is_err());
                prop_assert_eq!(model_src, std_src);
                if std_res.is_ok() {
                    prop_assert_eq!(model_buf, std_buf);
                }
            }

            #[test]
            fn test_write(
                init in prop::collection::vec(any::<u8>(), 0..50),
                data in prop::collection::vec(any::<u8>(), 0..50),
            ) {
                let mut model = init.clone();
                let model_n = Write::write(&mut model, &data);

                let mut std_v = init.clone();
                let std_n = std::io::Write::write(&mut std_v, &data);

                prop_assert_eq!(model_n.ok(), std_n.ok());
                prop_assert_eq!(model, std_v);
            }

            #[test]
            fn test_write_all(
                init in prop::collection::vec(any::<u8>(), 0..50),
                data in prop::collection::vec(any::<u8>(), 0..50),
            ) {
                let mut model = init.clone();
                prop_assert!(Write::write_all(&mut model, &data).is_ok());

                let mut std_v = init.clone();
                prop_assert!(std::io::Write::write_all(&mut std_v, &data).is_ok());

                prop_assert_eq!(model, std_v);
            }

            #[test]
            fn test_flush(init in prop::collection::vec(any::<u8>(), 0..50)) {
                let mut model = init.clone();
                let model_res = Write::flush(&mut model);

                let mut std_v = init.clone();
                let std_res = std::io::Write::flush(&mut std_v);

                prop_assert_eq!(model_res.is_err(), std_res.is_err());
                prop_assert_eq!(model, std_v);
            }
        }
    }
}
