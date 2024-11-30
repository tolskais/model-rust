use std::fs;

// Assume little endians
fn read_u16(vec: &[u8]) -> u16 {
    ((vec[0] as u16) << 8) | (vec[1] as u16)
}

fn main() {
    let contents = fs::read("test.class")
        .expect("asdf");

    let slice = &contents[8..10];
    let pool_count = read_u16(slice);
}
