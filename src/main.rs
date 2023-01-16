pub mod value;
use crate::value::Value;

fn main() {
    let a: Value<f32> = Value::new(2.0);
    let b: Value<f32> = Value::new(-3.0);
    let c: Value<f32> = Value::new(10.0);

    let d = a * b + c;

    for child in d.prev {
        println!("{}", child.data);
    }
    // println!("Hello, world!, {:?}", d.prev);
}
