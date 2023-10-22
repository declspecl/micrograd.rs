use super::{value::Value, operations::Operations, reproductive_method::ReproductiveMethod, constants};

use std::{rc::Rc, ops::{Add, Sub, Mul, Div}, cell::RefCell};

// ------------------------------------
// - RcValueParents convenience alias -
// ------------------------------------

type InternallyMutableValue<T> = Rc< RefCell< Value<T> > >;

// ----------------------
// - RcValue definition -
// ----------------------

#[derive(Debug, Clone)]
pub struct RcValue<T>
    where T: Copy + Into<f64>
{
    pub value: InternallyMutableValue<T>,
    pub operation: Option<Operations>,

    parents: Option< ReproductiveMethod< Box< RcValue<T> > > >
}

// --------------------------
// - RcValue implementation -
// --------------------------

impl<T> RcValue<T>
    where T: Copy + Into<f64>
{
    pub fn new<U>(value: U, parents: Option< ReproductiveMethod< Box< RcValue<T> > > >) -> Self
        where U: Into<Value<T>>
    {
        return Self
        {
            value: Rc::new(RefCell::new(value.into())),
            operation: None,
            parents
        };
    }

    pub fn from_operation<U>(value: U, operation: Operations, parents: Option< ReproductiveMethod< Box< RcValue<T> > > >) -> Self
        where U: Into<Value<T>>
    {
        return Self
        {
            value: Rc::new(RefCell::new(value.into())),
            operation: Some(operation),
            parents
        };
    }

    pub fn back_prop(&self)
    {
        self.value.borrow_mut().grad = 1.0;

        if let Some(ref operation) = self.operation
        {
            if let Some(ref parents) = self.parents
            {
                match parents
                {
                    ReproductiveMethod::Asexual(ref _parent) => match operation
                    {
                        Operations::Relu => {},
                        Operations::Tanh => {},
                        _ => {}
                    },
                    ReproductiveMethod::Sexual(ref left_parent, ref right_parent) => match operation
                    {
                        Operations::Add =>
                        {
                            left_parent.value.borrow_mut().grad += self.value.borrow().grad;
                            right_parent.value.borrow_mut().grad += self.value.borrow().grad;
                        },
                        Operations::Sub =>
                        {
                            left_parent.value.borrow_mut().grad += self.value.borrow().grad;
                            right_parent.value.borrow_mut().grad -= self.value.borrow().grad;
                        },
                        Operations::Mul =>
                        {
                            left_parent.value.borrow_mut().grad += self.value.borrow().grad * right_parent.value.borrow().data.into();
                            right_parent.value.borrow_mut().grad += self.value.borrow().grad * left_parent.value.borrow().data.into();
                        },
                        Operations::Div =>
                        {
                            left_parent.value.borrow_mut().grad += self.value.borrow().grad * (1.0 / right_parent.value.borrow().data.into());
                            right_parent.value.borrow_mut().grad += self.value.borrow().grad * -(left_parent.value.borrow().data.into() /
                                (right_parent.value.borrow().data.into() * (right_parent.value.borrow().data.into() + constants::H)));
                        },
                        _ => {}
                    }
                }
            }
        }
    }
}

// -----------------------
// - RcValue conversions -
// -----------------------

impl From<f64> for RcValue<f64>
{
    fn from(value: f64) -> Self
    {
        return Self::new(value, None);
    }
}

// ----------------------
// - RcValue operations -
// ----------------------
impl<T> Add for RcValue<T>
    where T: Add<Output = T> + Copy + Clone + Into<f64>
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output
    {
        return Self::from_operation(
            self.value.borrow().data + rhs.value.borrow().data,
            Operations::Add,
            Some(
                ReproductiveMethod::Sexual(
                    Box::new(self.clone()),
                    Box::new(rhs.clone())
                )
            )
        );
    }
}

impl<T> Sub for RcValue<T>
    where T: Sub<Output = T> + Copy + Clone + Into<f64>
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output
    {
        return Self::from_operation(
            self.value.borrow().data - rhs.value.borrow().data,
            Operations::Sub,
            Some(
                ReproductiveMethod::Sexual(
                    Box::new(self.clone()),
                    Box::new(rhs.clone())
                )
            )
        );
    }
}

impl<T> Mul for RcValue<T>
    where T: Mul<Output = T> + Copy + Clone + Into<f64>
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output
    {
        return Self::from_operation(
            self.value.borrow().data * rhs.value.borrow().data,
            Operations::Mul,
            Some(
                ReproductiveMethod::Sexual(
                    Box::new(self.clone()),
                    Box::new(rhs.clone())
                )
            )
        );
    }
}

impl<T> Div for RcValue<T>
    where T: Div<Output = T> + Copy + Clone + Into<f64>
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output
    {
        return Self::from_operation(
            self.value.borrow().data / rhs.value.borrow().data,
            Operations::Div,
            Some(
                ReproductiveMethod::Sexual(
                    Box::new(self.clone()),
                    Box::new(rhs.clone())
                )
            )
        );
    }
}
