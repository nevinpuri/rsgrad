pub mod value;
pub mod visualize;

use crate::value::Value;

fn main() {
    let a: Value<f32> = Value::with_label(2.0, "a");
    let b: Value<f32> = Value::with_label(-3.0, "b");
    let c: Value<f32> = Value::with_label(10.0, "c");

    let d = a * b + c;

    println!("{:#?}", d);
    // trace_console(d);

    // for child in d.prev {
    //     println!("{}", child.data);
    // }

    // println!("{:?}", d.op);
    // println!("Hello, world!, {:?}", d.prev);
}
