use super::value::Value;

use std::{rc::Rc, fmt, ops::{Add, Sub, Mul, Div}};

pub struct RcValue<T>(Rc< Value<T> >);

impl<T> RcValue<T>
{
    pub fn new(value: Value<T>) -> Self
    {
        return RcValue(Rc::new(value));
    }
}

impl<T> Add for RcValue<T>
    where T: std::ops::Add<Output = T> + Copy + Clone
{
    type Output = RcValue<T>;

    fn add(self, rhs: Self) -> Self::Output
    {
        return Self::new(
            Value {
                data: self.0.data + rhs.0.data,
                grad: 0.0,
                children: (
                    Some(self.0.clone()),
                    Some(rhs.0.clone())
                )
            }
        );
    }
}

impl<T> fmt::Display for RcValue<T>
    where T: fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        return write!(f, "{}", self.0.data);
    }
}
