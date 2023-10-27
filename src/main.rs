pub mod engine;

use engine::RcValue;

fn main()
{
    let a: RcValue<f64> = RcValue::new(200f64);

    let b = a.clone() * 10f64.into();

    b.grad_parents();

    println!("{b:#?}");
}
