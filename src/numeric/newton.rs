const TOLERANCE: f64 = 0.0000001;
const MAX_ITER: usize = 50;

pub fn newton(f: impl Fn(f64) -> f64, f_prime: impl Fn(f64) -> f64) -> Option<f64> {
    let mut x = 1f64;
    for _ in 1..MAX_ITER {
        let y = f(x);
        let y_prime = f_prime(x);

        if y_prime.abs() < TOLERANCE {
            return None
        }

        let x_i = x - (y/y_prime);
        if (x-x_i).abs() < TOLERANCE {
            return Some(x_i)
        }

        x = x_i;
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn test_find_zero_of_polynomial() {
        let mut rng = rand::thread_rng();

        for _ in 0..10000 {
            let a: f64 = rng.gen_range(-1000.0..1000.0);
            let b: f64 = rng.gen_range(-1000.0..1000.0);
            let c: f64 = rng.gen_range(-1000.0..1000.0);

            let f = |x: f64| {a*x*x + b*x + c};
            let f_prime = |x: f64| {2.0*a*x + b};

            if (b*b) - (4.0*a*c) < 0.0 {
                // f has no root
                match newton(f, f_prime) {
                    None => (),
                    Some(wrong_zero) => assert!(false, "returned wrong zero={}, f(zero)={}", wrong_zero, f(wrong_zero)),
                }
            } else {
                // f has at least one root
                match newton(f, f_prime) {
                    None => assert!(false, "no zero found while one exists, a={}, b={}, c={}", a, b, c),
                    Some(zero)=> assert!(f(zero).abs() < TOLERANCE, "zero={}, f(zero)={}, a={}, b={}, c={}", zero, f(zero), a, b, c),
                }
            }
        }
    }
}