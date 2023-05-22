#[derive(Debug)]
struct DivisionByZeroError;

fn safe_devision(dividend: f64, divisor: f64) -> Result<f64, DivisionByZeroError> {
    if divisor == 0.0 {
        Err(DivisionByZeroError)
    } else {
        Ok(dividend / divisor)
    }
}

#[test]
fn test_safe_divicsion() {
    println!("{:?}", safe_devision(8.0, 3.0));
    println!("{:?}", safe_devision(6.0, 3.0));
    println!("{:?}", safe_devision(8.0, 0.0));
}
