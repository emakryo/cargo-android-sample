// #[cfg_attr(target_os = "android", ndk_glue::main(logger(level="info", tag="cargo-android-sample")))]
pub fn main() {
    println!("Hello world");
    println!("OS Info: {}", os_info::get());
}

pub fn fibonacci_exp(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci_exp(n-1) + fibonacci_exp(n-2),
    }
}

pub fn fibonacci_lin(n: u64) -> u64 {
    (0..n).fold((1, 1), |(x0, x1), _| (x0+x1, x0)).1
}

pub fn fibonacci_log(n: u64) -> u64 {
    fn mul(x: [u64; 4], y: [u64; 4]) -> [u64; 4] {
        [x[0]*y[0]+x[1]*y[2], x[0]*y[1]+x[1]*y[3], x[2]*y[0]+x[3]*y[2], x[2]*y[1]+x[3]*y[3]]
    }

    fn rec(xs: [u64; 4], n: u64) -> [u64; 4] {
        match n {
            0 => [1, 0, 0, 1],
            n if n%2 == 0 => {
                let ys = rec(xs, n/2);
                mul(ys, ys)
            }
            n => {
                mul(rec(xs, n-1), xs)
            }
        }
    }
    let ys = rec([1, 1, 1, 0], n);
    ys[2] + ys[3]
}

#[cfg(test)]
mod tests {
    use crate::{fibonacci_exp, fibonacci_lin, fibonacci_log};

    #[test]
    fn unit_test() {
        assert_eq!(1 + 1, 2);
    }

    #[test]
    fn test_fibonacci_exp() {
        assert_eq!(fibonacci_exp(10), 89);
    }

    #[test]
    fn test_fibonacci_lin() {
        assert_eq!(fibonacci_lin(10), 89);
    }

    #[test]
    fn test_fibonacci_log() {
        assert_eq!(fibonacci_log(10), 89);
    }
}
