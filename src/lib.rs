#[derive(Debug, PartialEq)]
pub struct Dollar {
    amount: i32,
}

impl Dollar {
    pub fn new(amount: i32) -> Dollar {
        Dollar { amount: amount }
    }

    pub fn times(&self, multiplier: i32) -> Dollar {
        Dollar {
            amount: self.amount * multiplier,
        }
    }

    pub fn equals(&self, object: Dollar) -> bool {
        let dollar = object;
        self.amount == dollar.amount
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiplication() {
        let five = Dollar::new(5);
        assert_eq!(Dollar::new(10), five.times(2));
        assert_eq!(Dollar::new(15), five.times(3));
    }

    #[test]
    fn test_equality() {
        assert!(Dollar::new(5).equals(Dollar::new(5)));
        assert_eq!(false, Dollar::new(5).equals(Dollar::new(6)));
    }
}
