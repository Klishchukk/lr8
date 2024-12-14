pub fn min_positive(arr: &[i32]) -> Option<i32> {
    arr.iter().filter(|&&x| x > 0).min().copied()
}

pub fn sum_negative(arr: &[i32]) -> i32 {
    arr.iter().filter(|&&x| x < 0).sum()
}

pub fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

pub fn electric_current(voltage: f64, resistance: f64) -> Option<f64> {
    if resistance > 0.0 {
        Some(voltage / resistance)
    } else {
        None
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_positive() {
        let arr = [3, -1, 5, 0, 2];
        assert_eq!(min_positive(&arr), Some(2));
        assert_eq!(min_positive(&[-1, -2, -3]), None);
    }

    #[test]
    fn test_sum_negative() {
        let arr = [3, -1, 5, -2, -3];
        assert_eq!(sum_negative(&arr), -6);
        assert_eq!(sum_negative(&[3, 5, 7]), 0);
    }

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(10), 55);
    }

    #[test]
    fn test_electric_current() {
        assert_eq!(electric_current(10.0, 5.0), Some(2.0));
        assert_eq!(electric_current(10.0, 0.0), None);
    }
}

