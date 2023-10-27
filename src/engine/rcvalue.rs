use super::{value::Value, operations::{SexualOperation, ASexualOperation}, reproduction_method::ReproductionMethod, constants};

use std::{rc::Rc, ops::{Add, Sub, Mul, Div, Neg}, cell::RefCell};

// ------------------------------------
// - RcValueParents convenience alias -
// ------------------------------------

type BoxedRcValue<T> = Box< RcValue<T> >;
type InternallyMutableValue<T> = Rc< RefCell< Value<T> > >;

// ----------------------
// - RcValue definition -
// ----------------------

#[derive(Debug, Clone)]
pub struct RcValue<T>
    where T: Copy + Into<f64> + From<f64>
{
    pub value: InternallyMutableValue<T>,
    pub lineage: Option< ReproductionMethod< BoxedRcValue<T> > >
}

// --------------------------
// - RcValue implementation -
// --------------------------

impl<T> RcValue<T>
    where T: Copy + Into<f64> + From<f64>
{
    pub fn new<U>(value: U) -> Self
        where U: Into<Value<T>>
    {
        return Self
        {
            value: Rc::new(RefCell::new(value.into())),
            lineage: None
        };
    }

    pub fn from_operation<U>(value: U, lineage: Option< ReproductionMethod< BoxedRcValue<T> > >) -> Self
        where U: Into<Value<T>>
    {
        return Self
        {
            value: Rc::new(RefCell::new(value.into())),
            lineage
        };
    }

    pub fn tanh(&self) -> Self
    {
        return Self::from_operation(
            Into::<T>::into(self.value.borrow().data.into().tanh()),
            Some(
                ReproductionMethod::Asexual(
                    ASexualOperation::Tanh,
                    Box::new(self.clone())
                )
            )
        );
    }

    pub fn grad_parents(&self)
    {
        if let Some(ref lineage) = self.lineage
        {
            match lineage
            {
                ReproductionMethod::Sexual(ref operation, ref parents) =>
                {
                    parents.0.value.borrow_mut().grad = 0.0;
                    parents.1.value.borrow_mut().grad = 0.0;

                    match operation
                    {
                        SexualOperation::Add =>
                        {
                            parents.0.value.borrow_mut().grad += self.value.borrow().grad;
                            parents.0.value.borrow_mut().grad += self.value.borrow().grad;
                        },
                        SexualOperation::Sub =>
                        {
                            parents.0.value.borrow_mut().grad += self.value.borrow().grad;
                            parents.0.value.borrow_mut().grad -= self.value.borrow().grad;
                        },
                        SexualOperation::Mul =>
                        {
                            parents.0.value.borrow_mut().grad += self.value.borrow().grad * parents.0.value.borrow().data.into();
                            parents.0.value.borrow_mut().grad += self.value.borrow().grad * parents.0.value.borrow().data.into();
                        },
                        SexualOperation::Div =>
                        {
                            parents.0.value.borrow_mut().grad += self.value.borrow().grad * (1.0 / parents.0.value.borrow().data.into());
                            parents.0.value.borrow_mut().grad += self.value.borrow().grad * -(parents.0.value.borrow().data.into() /
                                (parents.0.value.borrow().data.into() * (parents.0.value.borrow().data.into() + constants::H)));
                        }
                    }
                },
                ReproductionMethod::Asexual(ref operation, ref parent) =>
                {
                    parent.value.borrow_mut().grad = 0.0;

                    match operation
                    {
                        ASexualOperation::Relu =>
                        {
                            parent.value.borrow_mut().grad += if parent.value.borrow().data.into() > 0.0 { self.value.borrow().grad } else { 0.0 };
                        }
                        ASexualOperation::Tanh => {
                            parent.value.borrow_mut().grad += -1f64 - (self.value.borrow().data.into() * self.value.borrow().data.into());
                        },
                        ASexualOperation::Neg =>
                        {
                            parent.value.borrow_mut().grad += -1f64;
                        }
                    }
                }
            }
        }
    }

    pub fn parameters(&self) -> Vec< InternallyMutableValue<T> >
    {
        let mut parameters: Vec< InternallyMutableValue<T> > = Vec::with_capacity(100);

        let mut value_queue: Vec<>

        return parameters;
    }
}

// -----------------------
// - RcValue conversions -
// -----------------------

impl<T> From<T> for RcValue<T>
    where T: Copy + Into<f64> + From<f64>
{
    fn from(value: T) -> Self
    {
        return Self::new(value);
    }
}

// ----------------------
// - RcValue operations -
// ----------------------
impl<T> Add for RcValue<T>
    where T: Add<Output = T> + Copy + Clone + Into<f64> + From<f64>
{
    type Output = Self;

    fn add(self, rhs: Self::Output) -> Self::Output
    {
        return Self::Output::from_operation(
            self.value.borrow().data + rhs.value.borrow().data,
            Some(
                ReproductionMethod::Sexual(
                    SexualOperation::Add,
                    (
                        Box::new(self.clone()),
                        Box::new(rhs.clone())
                    )
                )
            )
        );
    }
}

impl<T> Sub for RcValue<T>
    where T: Sub<Output = T> + Copy + Clone + Into<f64> + From<f64>
{
    type Output = Self;

    fn sub(self, rhs: Self::Output) -> Self::Output
    {
        return Self::Output::from_operation(
            self.value.borrow().data - rhs.value.borrow().data,
            Some(
                ReproductionMethod::Sexual(
                    SexualOperation::Sub,
                    (
                        Box::new(self.clone()),
                        Box::new(rhs.clone())
                    )
                )
            )
        );
    }
}

impl<T> Mul for RcValue<T>
    where T: Mul<Output = T> + Copy + Clone + Into<f64> + From<f64>
{
    type Output = Self;

    fn mul(self, rhs: Self::Output) -> Self::Output
    {
        return Self::Output::from_operation(
            self.value.borrow().data * rhs.value.borrow().data,
            Some(
                ReproductionMethod::Sexual(
                    SexualOperation::Mul,
                    (
                        Box::new(self.clone()),
                        Box::new(rhs.clone())
                    )
                )
            )
        );
    }
}

impl<T> Div for RcValue<T>
    where T: Div<Output = T> + Copy + Clone + Into<f64> + From<f64>
{
    type Output = Self;

    fn div(self, rhs: Self::Output) -> Self::Output
    {
        return Self::Output::from_operation(
            self.value.borrow().data / rhs.value.borrow().data,
            Some(
                ReproductionMethod::Sexual(
                    SexualOperation::Div,
                    (
                        Box::new(self.clone()),
                        Box::new(rhs.clone())
                    )
                )
            )
        );
    }
}

impl<T> Neg for RcValue<T>
    where T: Neg<Output = T> + Copy + Clone + Into<f64> + From<f64>
{
    type Output = Self;

    fn neg(self) -> Self::Output
    {
        return Self::Output::from_operation(
            -self.value.borrow().data,
            Some(
                ReproductionMethod::Asexual(
                    ASexualOperation::Neg,
                    Box::new(self.clone())
                )
            )
        )
    }
}
