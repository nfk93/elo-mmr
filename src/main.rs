mod numeric;
use numeric::newton::newton;

fn main() {
    println!("Hello, world!");

    let f = |x| {x*x-5.0};
    let f_prime = |x| {2f64*x};

    let zero = newton(f, f_prime).unwrap();
    println!("{}", zero);
    println!("{}", f(zero));

    let f = |x: f64| {15.0*x*x + 201.0};
    let f_prime = |x: f64| {30.0*x};

    let zero = newton(f, f_prime);
    println!("{:?}", zero);
}