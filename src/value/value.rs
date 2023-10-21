use std::rc::Rc;

// --------------------
// - Value definition -
// --------------------

pub struct Value<T>
{
    pub data: T,
    pub grad: f64,

    pub children: (Option< Rc< Value<T> > >, Option< Rc< Value<T> > >)
}

// ------------------------
// - Value implementation -
// ------------------------

impl<T> Value<T>
{
    pub fn new(data: T) -> Self
    {
        return Self { data , grad: 0.0, children: (None, None) };
    }
}

// ---------------------
// - Value conversions -
// ---------------------

impl<T> From<T> for Value<T>
{
    fn from(value: T) -> Self
    {
        return Value { data: value, grad: 0.0, children: (None, None) };
    }
}

