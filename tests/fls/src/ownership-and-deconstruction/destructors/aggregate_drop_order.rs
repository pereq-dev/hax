//! @fls: fls_kdqng6eovxns, fls_jzancf72i95f, fls_sup43es8ps8r, fls_tw36n3g32a0y
//! @title: The fields of a tuple and a struct and the elements of an array drop in order
//! @lean(tier): unsupported
//! @lean(reason): The ownership model and the value-model boundary of the input-language reference: a destructor whose body has an observable effect has no faithful reading, and the `Cell` that records the order is interior mutability. charon and aeneas accept the crate, and only the missing model of `Cell` stops it at `lake build` (`Unknown identifier core.cell.Cell`).

use core::cell::Cell;

pub struct Logger<'a> {
    pub log: &'a Cell<u32>,
    pub tag: u32,
}

impl Drop for Logger<'_> {
    fn drop(&mut self) {
        self.log.set(self.log.get() * 10 + self.tag);
    }
}

pub fn tuple_order() -> u32 {
    let log = Cell::new(0);
    {
        let _pair = (Logger { log: &log, tag: 1 }, Logger { log: &log, tag: 2 });
    }
    log.get()
}

pub struct Pair<'a> {
    pub first: Logger<'a>,
    pub second: Logger<'a>,
}

pub fn struct_order() -> u32 {
    let log = Cell::new(0);
    {
        let _pair = Pair {
            first: Logger { log: &log, tag: 1 },
            second: Logger { log: &log, tag: 2 },
        };
    }
    log.get()
}

pub fn array_order() -> u32 {
    let log = Cell::new(0);
    {
        let _all = [
            Logger { log: &log, tag: 1 },
            Logger { log: &log, tag: 2 },
            Logger { log: &log, tag: 3 },
        ];
    }
    log.get()
}

#[hax_test::equiv]
pub fn drops_in_order() -> bool {
    tuple_order() == 12 && struct_order() == 12 && array_order() == 123
}
