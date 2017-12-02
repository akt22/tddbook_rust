// ==================================
// IMoney
// ----------------------------------
pub trait IMoney {
    fn amount(&self) -> i32;
    fn currency(&self) -> &String;

    fn equals<T: IMoney>(&self, object: T) -> bool {
        self.amount() == object.amount() && self.currency() == object.currency()
    }
}

pub struct Money {
    amount: i32,
    currency: String,
}

impl IMoney for Money {
    fn amount(&self) -> i32 {
        self.amount
    }

    fn currency(&self) -> &String {
        &self.currency
    }
}

// ==================================
// Dollar
// ----------------------------------
#[derive(Debug, PartialEq)]
pub struct Dollar {
    amount: i32,
    currency: String,
}

impl IMoney for Dollar {
    fn amount(&self) -> i32 {
        self.amount
    }

    fn currency(&self) -> &String {
        &self.currency
    }
}

impl Dollar {
    pub fn new(amount: i32) -> Dollar {
        Dollar {
            amount: amount,
            currency: "USD".to_string(),
        }
    }

    pub fn times(&self, multiplier: i32) -> Dollar {
        Dollar {
            amount: self.amount * multiplier,
            currency: "USD".to_string(),
        }
    }
}

// ==================================
// Franc
// ----------------------------------
#[derive(Debug, PartialEq)]
pub struct Franc {
    amount: i32,
    currency: String,
}

impl IMoney for Franc {
    fn amount(&self) -> i32 {
        self.amount
    }

    fn currency(&self) -> &String {
        &self.currency
    }
}

impl Franc {
    pub fn new(amount: i32) -> Franc {
        Franc {
            amount: amount,
            currency: "CHF".to_string(),
        }
    }

    pub fn times(&self, multiplier: i32) -> Franc {
        Franc {
            amount: self.amount * multiplier,
            currency: "CHF".to_string(),
        }
    }
}

// ==================================
// Test
// ----------------------------------
#[cfg(test)]
mod tests {
    use super::Dollar;
    use super::Franc;
    use super::IMoney;
    use super::Money;

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
        assert!(Franc::new(5).equals(Franc::new(5)));
        assert_eq!(false, Franc::new(5).equals(Franc::new(6)));
        assert_eq!(false, Franc::new(5).equals(Dollar::new(5)));
    }

    #[test]
    fn test_franc_multiplication() {
        let five = Franc::new(5);
        assert_eq!(Franc::new(10), five.times(2));
        assert_eq!(Franc::new(15), five.times(3));
    }
}
