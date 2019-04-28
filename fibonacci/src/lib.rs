/// Basic recursive fibonacci implementation
fn fib(n: i64) -> i64 {
    if n < 2 {
        // base case
        return n;
    }
    return fib(n - 2) + fib(n - 1); // Recursive case
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_fib() {
        assert_eq!(55, fib(10));
    }
}
