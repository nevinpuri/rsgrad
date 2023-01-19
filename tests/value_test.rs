use rsgrad::value::Value;

#[test]
fn test_grad() {
    let h = 0.00001;

    let a: Value<f32> = Value::with_label(2.0, "a");
    let b: Value<f32> = Value::with_label(-3.0, "b");
    let c: Value<f32> = Value::with_label(10.0, "c");

    let mut e = a * b;
    e.label = "e";
    let d = e + c;
    let f = Value::with_label(-2.0, "f");

    let mut l = d * f;
    l.label = "l";
    let l1 = l.data;

    // we're adding h in this second part
    let a: Value<f32> = Value::with_label(2.0, "a") + h;
    let b: Value<f32> = Value::with_label(-3.0, "b");
    let c: Value<f32> = Value::with_label(10.0, "c");

    let mut e = a * b;
    e.label = "e";
    let d = e + c;
    let f = Value::with_label(-2.0, "f");
    let mut l = d * f;
    l.label = "l";

    let l2 = l.data;

    let derivative = (l2 - l1) / h;

    println!("{:.2}", (l2 - l1) / h);
    assert_eq!(format!("{:.2}", derivative), "6.01");

    // calculating derivatives
    // L
    // what is the derivative of L with respect to L

    // assert_eq!(d.data, 4.0)
    // trace_console(d);

    // for child in d.prev {
    //     println!("{}", child.data);
    // }

    // println!("{:?}", d.op);
    // println!("Hello, world!, {:?}", d.prev);
}
