use std::ops::Range;

// Show the constant table using sin function.
pub fn show_constant_table() {
    for e in create_constant_table(0..64) {
        println!("0x{:08x}", e);
    }
}

// Create elements of the constant table using sin function.
fn create_constant_table(range: Range<i32>) -> Vec<u32> {
    let mut elements: Vec<u32> = Vec::new();
    for i in range {
        elements.push(calculate_constant_table(i));
    }
    elements
}

// Calculate the ⌊2^32 * abs(sin(i))⌋ for i in range(64).
fn calculate_constant_table(i: i32) -> u32 {
    (2.0_f64.powf(32.0) * (i as f64).sin().abs()) as u32
}
