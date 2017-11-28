#[derive(Debug)]
pub struct Dollar {
    amount: i32,
}

impl Dollar {
    fn new(amount: i32) -> Dollar {
        Dollar { amount: amount }
    }
    fn times(&mut self, multiplier: i32) {
        self.amount *= multiplier;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiplication() {
        let mut five = Dollar::new(5);
        five.times(2);
        assert_eq!(10, five.amount);
    }
}
