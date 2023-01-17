use num::Float;
use std::{fmt, ops};

/// TODO: replace this with standard library operation enum

#[derive(Clone, Debug, PartialEq)]
pub enum Operation {
    Add,
    Sub,
    Mul,
    Div,
    Create,
}

/// TODO: make default type be f32, not sure why it isn't working right now
#[derive(Clone, Debug)]
pub struct Value<'a, T = f32>
where
    T: Float,
{
    pub label: &'a str,
    pub data: T,
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
            prev: Vec::new(),
            op: Operation::Create,
        }
    }

    pub fn with_label(val: T, label: &'a str) -> Self {
        Value {
            label: label,
            data: val,
            prev: Vec::new(),
            op: Operation::Create,
        }
    }

    pub fn with_children(val: T, children: Vec<Value<'a, T>>) -> Self {
        Value {
            label: "",
            data: val,
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
            prev: vec![self, rhs],
            op: Operation::Div,
        }
    }
}
