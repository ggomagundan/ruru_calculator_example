#[macro_use]
extern crate ruru;

use std::error::Error;
use ruru::{Class, Fixnum, Hash, Object, VM};

class!(RustyCalculator);

methods!(
    RustyCalculator,
    itself,

    fn pow_3(num: Fixnum) -> Hash {
        let mut result = Hash::new();

        // Raise an exception if `number` is not a Fixnum
        if let Err(ref error) = num {
            VM::raise(error.to_exception(), error.description());
        }

        for i in 1..num.unwrap().to_i64() + 1 {
            result.store(Fixnum::new(i), Fixnum::new(i.pow(3)));
        }

        result
    }
);

#[no_mangle]
pub extern fn initialize_my_app() {
    Class::new("RustyCalculator", None).define(|itself| {
        itself.def("pow_3", pow_3);
    });
}
