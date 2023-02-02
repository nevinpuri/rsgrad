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
    let a: Value<f32> = Value::with_label(2.0, "a");
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

    // grads
    // l.grad = 1.0;
    // d.grad = f.data;
    // f.grad = d.data;

    // assert_eq!(format!("{:.2}", derivative), "6.01");

    // l = d * f
    // what is dl / dd
    //
    // (f(x+h) - f(x)) / h
    // L = d * f
    // increasing d by h
    // (((d+h) * f) - (d * f)) / h
    // (d * f + h * f - d * f) / h
    // (h * f) / h
    // f

    // dl / df = d

    println!("hello");

    // calculating derivatives

    // + routes derivative

    // e = a * b
    // de / da = b

    // dL / da = (dL / de) * (de / da)
}
