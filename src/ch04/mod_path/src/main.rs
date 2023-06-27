mod aaa {
    pub mod bbb {
        pub mod ccc {
            pub fn print() {
                println!("aaa::bbb::ccc::print()");
            }
        }
        pub mod ddd {
            pub fn print() {
                println!("aaa::bbb::ddd::print()");
            }
        }
    }
}

use aaa::bbb::ccc::print as print_c;
use aaa::bbb::{ccc, ddd};

fn main() {
    ccc::print();
    aaa::bbb::ccc::print();
    crate::aaa::bbb::ddd::print();
    print_c();
}
