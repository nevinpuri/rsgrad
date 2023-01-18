use num::Float;
use std::{fmt, ops};

/// An operation from which a value was created from
#[derive(Clone, Debug, PartialEq)]
pub enum Operation {
    Add,
    Sub,
    Mul,
    Div,
    Create,
}

/// A value instance
#[derive(Clone, Debug)]
pub struct Value<'a, T = f32>
where
    T: Float,
{
    pub label: &'a str,
    pub data: T,
    // warning: type T might not be the correct type for grad, maybe just use f16 or f32
    pub grad: T,
    pub op: Operation,
    pub prev: Vec<Value<'a, T>>,
}

impl<'a, T> Value<'a, T>
where
    T: Float,
{
    pub fn new(val: T) -> Self {
        Value {
            label: "",
            data: val,
            // warning: this grad implementation might not be good, maybe just use f32 or f16. Not sure how this stuff works so it will be zero for now
            grad: num::zero(),
            prev: Vec::new(),
            op: Operation::Create,
        }
    }

    pub fn with_label(val: T, label: &'a str) -> Self {
        Value {
            label: label,
            data: val,
            grad: num::zero(),
            prev: Vec::new(),
            op: Operation::Create,
        }
    }

    pub fn with_children(val: T, children: Vec<Value<'a, T>>) -> Self {
        Value {
            label: "",
            data: val,
            grad: num::zero(),
            prev: children,
            op: Operation::Create,
        }
    }
}

impl<'a, T> fmt::Display for Value<'a, T>
where
    T: Float + std::fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "value: {:#?}, children: {:?}", self.data, self.prev)
    }
}

impl<'a, T> ops::Add<T> for Value<'a, T>
where
    T: Float + ops::AddAssign,
{
    type Output = Value<'a, T>;
    fn add(self, rhs: T) -> Self::Output {
        Value {
            label: "",
            data: self.data + rhs,
            grad: num::zero(),
            prev: vec![self, Value::new(rhs)],
            op: Operation::Add,
        }
    }
}

impl<'a, T> ops::Sub<T> for Value<'a, T>
where
    T: Float + ops::SubAssign,
{
    type Output = Value<'a, T>;
    fn sub(self, rhs: T) -> Self::Output {
        Value {
            label: "",
            data: self.data - rhs,
            grad: num::zero(),
            prev: vec![self, Value::new(rhs)],
            op: Operation::Sub,
        }
    }
}

impl<'a, T> ops::Mul<T> for Value<'a, T>
where
    T: Float + ops::MulAssign,
{
    type Output = Value<'a, T>;
    fn mul(self, rhs: T) -> Self::Output {
        Value {
            label: "",
            data: self.data * rhs,
            grad: num::zero(),
            prev: vec![self, Value::new(rhs)],
            op: Operation::Mul,
        }
    }
}

impl<'a, T> ops::Div<T> for Value<'a, T>
where
    T: Float + ops::DivAssign,
{
    type Output = Value<'a, T>;
    fn div(self, rhs: T) -> Self::Output {
        Value {
            label: "",
            data: self.data / rhs,
            grad: num::zero(),
            prev: vec![self, Value::new(rhs)],
            op: Operation::Div,
        }
    }
}

impl<'a, T> ops::Add<Value<'a, T>> for Value<'a, T>
where
    T: Float,
{
    type Output = Value<'a, T>;
    fn add(self, rhs: Value<'a, T>) -> Self::Output {
        Value {
            label: "",
            data: self.data + rhs.data,
            grad: num::zero(),
            prev: vec![self, rhs],
            op: Operation::Add,
        }
    }
}

impl<'a, T> ops::Sub<Value<'a, T>> for Value<'a, T>
where
    T: Float,
{
    type Output = Value<'a, T>;
    fn sub(self, rhs: Value<'a, T>) -> Self::Output {
        Value {
            label: "",
            data: self.data - rhs.data,
            grad: num::zero(),
            prev: vec![self, rhs],
            op: Operation::Sub,
        }
    }
}

impl<'a, T> ops::Mul<Value<'a, T>> for Value<'a, T>
where
    T: Float,
{
    type Output = Value<'a, T>;
    fn mul(self, rhs: Value<'a, T>) -> Self::Output {
        Value {
            label: "",
            data: self.data * rhs.data,
            grad: num::zero(),
            prev: vec![self, rhs],
            op: Operation::Mul,
        }
    }
}

impl<'a, T> ops::Div<Value<'a, T>> for Value<'a, T>
where
    T: Float,
{
    type Output = Value<'a, T>;
    fn div(self, rhs: Value<'a, T>) -> Self::Output {
        Value {
            label: "",
            data: self.data / rhs.data,
            grad: num::zero(),
            prev: vec![self, rhs],
            op: Operation::Div,
        }
    }
}
