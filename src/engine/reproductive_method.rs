#[derive(Debug, Clone, Copy)]
pub enum ReproductiveMethod<T>
{
    Asexual(T),
    Sexual(T, T)
}
