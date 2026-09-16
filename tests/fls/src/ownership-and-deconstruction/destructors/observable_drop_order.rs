//! @fls: fls_4nkzidytpi6, fls_bync24y6gp93, fls_tw36n3g32a0y
//! @title: Destructors with an observable effect run in reverse declaration order
//! @lean(tier): unsupported
//! @lean(reason): The ownership model of the input-language reference: a destructor whose body has an observable effect has no faithful reading, since the value model has no effect at scope exit to attach it to. charon emits the `Drop` statement with the destructor, aeneas evaluates the statement as a no-op and never calls the user's `drop`, and only the elaboration of the `Drop` implementation stops the extraction (`Type mismatch: CoreOpsDropDrop.drop has type Logger → RustM (Logger × (Logger → Logger))`); a destructor over owned state would pass with its effect lost.

pub struct Logger<'a> {
    pub log: &'a mut [u8; 4],
    pub len: &'a mut usize,
    pub tag: u8,
}

impl Drop for Logger<'_> {
    fn drop(&mut self) {
        self.log[*self.len] = self.tag;
        *self.len += 1;
    }
}

pub fn run() -> [u8; 4] {
    let mut log = [0u8; 4];
    let mut len = 0;
    {
        let _a = Logger {
            log: &mut log,
            len: &mut len,
            tag: 1,
        };
    }
    {
        let first = Logger {
            log: &mut log,
            len: &mut len,
            tag: 2,
        };
        let _second = Logger {
            log: &mut *first.log,
            len: &mut *first.len,
            tag: 3,
        };
    }
    log
}

#[hax_test::equiv]
pub fn program_runs() -> bool {
    run() == [1, 3, 2, 0]
}
