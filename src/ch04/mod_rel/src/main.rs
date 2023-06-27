mod random {
    pub mod linear {
        pub fn rand() -> u32 {
            1
        }
    }

    pub mod xorshift {
        pub fn rand() -> u32 {
            //이부분에서 에러가 발생할까?
            return crate::linear::rand();
        }
    }
}
fn main() {
    
    println!("{}", crate::random::xorshift::rand());
}
