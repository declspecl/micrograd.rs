pub mod value;

use value::RcValue;

fn main()
{
    let a: RcValue<f64> = RcValue::new(10.0.into());
    let b: RcValue<f64> = RcValue::new(20.0.into());

    let c: RcValue<f64> = a + b;

    println!("{}", c);
}
