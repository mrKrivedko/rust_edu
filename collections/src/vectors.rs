pub const VECTOR1: Vec<i32> = Vec::new();

pub fn get_vector_123() -> Vec<i32> {
    vec![1, 2, 3, 4, 5]
}

#[derive(Debug)]
pub enum SpreadSheetsCell {
    Int(i32),
    Float(f32),
    Text(String),
}
