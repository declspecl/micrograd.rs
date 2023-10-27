use super::operations::{ASexualOperation, SexualOperation};

#[derive(Debug, Clone, Copy)]
pub enum ReproductionMethod<T>
{
    Asexual(ASexualOperation, T),
    Sexual(SexualOperation, (T, T))
}
