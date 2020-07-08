use super::value::Value;

#[derive(Debug, Default)]
pub struct Stack(Vec<Value>);

impl Stack {
    pub fn pop(&mut self) -> Value {
        match self.0.pop() {
            None => panic!("Stack::pop_i32: empty stack"),
            Some(val) => val,
        }
    }

    pub fn pop_i32(&mut self) -> i32 {
        match self.0.pop() {
            None => panic!("Stack::pop_i32: empty stack"),
            Some(Value::I32(val)) => val,
            Some(other) => panic!("Stack::pop_i32: {:#?}", other),
        }
    }

    pub fn push(&mut self, val: Value) {
        self.0.push(val)
    }

    pub fn push_i32(&mut self, i: i32) {
        self.0.push(Value::I32(i))
    }

    pub fn push_u32(&mut self, i: u32) {
        self.0.push(Value::I32(i as i32))
    }

    pub fn push_i64(&mut self, i: i64) {
        self.0.push(Value::I64(i))
    }

    pub fn push_f32(&mut self, f: f32) {
        self.0.push(Value::F32(f))
    }

    pub fn push_f64(&mut self, f: f64) {
        self.0.push(Value::F64(f))
    }

    pub fn push_bool(&mut self, bool: bool) {
        self.push_u32(if bool { 1 } else { 0 })
    }
}