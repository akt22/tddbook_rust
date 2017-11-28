#[derive(Debug)]
pub struct Dollar {
    amount: i32,
}

impl Dollar {
    fn new(amount: i32) -> Dollar {
        Dollar { amount: amount }
    }
    fn times(&self, multiplier: i32) -> Dollar {
        Dollar { amount: self.amount * multiplier }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiplication() {
        let five = Dollar::new(5);
        let product = five.times(2);
        assert_eq!(10, product.amount);
        let product = five.times(3);
        assert_eq!(15, product.amount);
    }
}
