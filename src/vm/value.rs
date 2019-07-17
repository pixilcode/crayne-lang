#[derive(PartialEq, Debug)]
pub enum Value {}

#[derive(PartialEq, Debug)]
pub struct ConstantPool(Vec<Value>);

impl ConstantPool {
    pub fn new() -> Self {
        ConstantPool(vec![])
    }
    
    pub fn write(mut self, value: Value) -> Self {
        self.0.push(value);
        self
    }
}