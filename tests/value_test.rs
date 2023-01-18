use rsgrad::value::Value;

#[test]
fn test_value() {
    let a: Value<f32> = Value::with_label(2.0, "a");
    let b: Value<f32> = Value::with_label(-3.0, "b");
    let c: Value<f32> = Value::with_label(10.0, "c");

    let mut e = a * b;
    e.label = "e";

    let d = e + c;
    let f = Value::with_label(-2.0, "f");

    let L = d * f;

    println!("{:#?}", L);
    // assert_eq!(d.data, 4.0)
    // trace_console(d);

    // for child in d.prev {
    //     println!("{}", child.data);
    // }

    // println!("{:?}", d.op);
    // println!("Hello, world!, {:?}", d.prev);
}
