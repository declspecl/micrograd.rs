pub mod engine;

use engine::RcValue;

fn main()
{
    let a: RcValue<f64> =
    (
        (
            Into::<RcValue<f64>>::into(10.0)
            +
            20.0.into()
        )
        *
        2.0.into()
    )
    -
    420.0.into();

    a.back_prop();

    println!("{a:#?}");
}
