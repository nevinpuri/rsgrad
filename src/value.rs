use num::Float;
use std::{fmt, ops};

#[derive(Clone, Copy, Debug)]
pub struct Value<T = f32>
where
    T: Float,
{
    data: T,
}

impl<T> Value<T>
where
    T: Float,
{
    pub fn new(val: T) -> Self {
        Value { data: val }
    }
}

impl<T> fmt::Display for Value<T>
where
    T: Float + std::fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self.data)
    }
}

impl<T> ops::Add<T> for Value<T>
where
    T: Float + ops::AddAssign,
{
    type Output = Value<T>;
    fn add(self, rhs: T) -> Self::Output {
        Value {
            data: self.data + rhs,
        }
    }
}

impl<T> ops::Sub<T> for Value<T>
where
    T: Float + ops::SubAssign,
{
    type Output = Value<T>;
    fn sub(self, rhs: T) -> Self::Output {
        Value {
            data: self.data - rhs
        }
    }
}

impl<T> ops::Mul<T> for Value<T>
where
    T: Float + ops::MulAssign,
{
    type Output = Value<T>;
    fn mul(self, rhs: T) -> Self::Output {
        Value {
            data: self.data * rhs,
        }
    }
}

impl<T> ops::Div<T> for Value<T>
where
    T: Float + ops::DivAssign
{
    type Output = Value<T>;
    fn div(self, rhs: T) -> Self::Output {
        Value {
            data: self.data / rhs
        }
    }
}

impl<T> ops::Add<Value<T>> for Value<T>
where
    T: Float,
{
    type Output = Value<T>;
    fn add(self, rhs: Value<T>) -> Self::Output {
        Value {
            data: self.data + rhs.data,
        }
    }
}

impl<T> ops::Sub<Value<T>> for Value<T>
where
    T: Float,
{
    type Output = Value<T>;
    fn sub(self, rhs: Value<T>) -> Self::Output {
        Value {
            data: self.data - rhs.data
        }
    }
}

impl<T> ops::Mul<Value<T>> for Value<T>
where
    T: Float,
{
    type Output = Value<T>;
    fn mul(self, rhs: Value<T>) -> Self::Output {
        Value {
            data: self.data * rhs.data,
        }
    }
}

impl<T> ops::Div<Value<T>> for Value<T>
where
    T: Float,
{
    type Output = Value<T>;
    fn div(self, rhs: Value<T>) -> Self::Output {
        Value {
            data: self.data / rhs.data
        }
    }
}
