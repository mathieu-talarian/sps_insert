use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Outputs(Vec<Output>);
impl Outputs {
    pub fn push(&mut self, output: Output) {
        self.0.push(output);
    }
    pub fn new() -> Self {
        Self(Vec::new())
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Output {
    value_to_string: String,
    integer: i32,
    unsigned_integer: u32,
    float: f32,
    boolean: bool,
}

impl Output {}

pub trait Create<T> {
    fn create(value: T) -> Self;
}
impl Create<i32> for Output {
    fn create(value: i32) -> Self {
        Self {
            value_to_string: value.to_string(),
            integer: value,
            unsigned_integer: value as u32,
            float: value as f32,
            boolean: value % 2 == 0,
        }
    }
}
impl Create<f32> for Output {
    fn create(value: f32) -> Self {
        Self {
            value_to_string: value.to_string(),
            integer: value as i32,
            unsigned_integer: value as u32,
            float: value,
            boolean: value % 2f32 == 0f32,
        }
    }
}
