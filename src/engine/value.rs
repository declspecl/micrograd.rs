// --------------------
// - Value definition -
// --------------------

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Value<T>
    where T: Copy + Into<f64>
{
    pub data: T,
    pub grad: f64
}

// ------------------------
// - Value implementation -
// ------------------------

impl<T> Value<T>
    where T: Copy + Into<f64>
{
    pub fn new(data: T) -> Self
    {
        return Self { data , grad: 0.0 };
    }
}

// ---------------------
// - Value conversions -
// ---------------------

impl<T> From<T> for Value<T>
    where T: Copy + Into<f64>
{
    fn from(value: T) -> Self
    {
        return Value { data: value, grad: 0.0 };
    }
}


// pub trait Value<T>
//     where T: Debug + Copy + Clone + PartialEq + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>
// {
//     type Output;
// 
//     fn new(data: T) -> Self::Output;
//     fn data(&self) -> T;
//     fn grad(&self) -> f64;
//     fn parents(&self) -> (Option<Self::Output>, Option<Self::Output>);
//     fn back_propagate<F>(&mut self, prop_fn: F) where F: FnMut(&mut Self::Output);
// }
