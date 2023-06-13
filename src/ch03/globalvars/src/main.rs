static mut TAX: f32 = 0.1;
fn main() {
    unsafe {
        println!("Tax is {}", TAX);
        TAX = 0.08;
        println!("Tax is {}", TAX);
    }
}
