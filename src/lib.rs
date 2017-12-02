// ==================================
// Money
// ----------------------------------
pub trait IMoney {
    fn amount(&self) -> i32;
    fn currency(&self) -> &String;
}

#[derive(Debug, PartialEq)]
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

impl Money {
    pub fn new(amount: i32, currency: String) -> Money {
        Money {
            amount: amount,
            currency: currency,
        }
    }

    pub fn dollar(amount: i32) -> Money {
        Money {
            amount: amount,
            currency: "USD".to_string(),
        }
    }

    pub fn franc(amount: i32) -> Money {
        Money {
            amount: amount,
            currency: "CHF".to_string(),
        }
    }

    // IMoney traitを実装している必要がある
    pub fn equals<T: IMoney>(&self, object: T) -> bool {
        self.amount() == object.amount() && self.currency() == object.currency()
    }

    pub fn times(&self, multiplier: i32) -> Money {
        Money::new(self.amount * multiplier, self.currency.to_string())
    }
}

// ==================================
// etc
// ----------------------------------
pub fn new_dollar(amount: i32) -> Money {
    Money {
        amount: amount,
        currency: "USD".to_string(),
    }
}

pub fn new_franc(amount: i32) -> Money {
    Money {
        amount: amount,
        currency: "CHF".to_string(),
    }
}

// ==================================
// Test
// ----------------------------------
#[cfg(test)]
mod tests {
    use super::IMoney;
    use super::Money;

    #[test]
    fn test_multiplication() {
        let five = Money::dollar(5);
        assert_eq!(Money::dollar(10), five.times(2));
        assert_eq!(Money::dollar(15), five.times(3));
    }

    #[test]
    fn test_equality() {
        assert!(Money::dollar(5).equals(Money::dollar(5)));
        assert_eq!(false, Money::dollar(5).equals(Money::dollar(6)));
        assert_eq!(false, Money::franc(5).equals(Money::dollar(5)));
    }

    #[test]
    fn test_currency() {
        assert_eq!("USD", Money::dollar(1).currency());
        assert_eq!("CHF", Money::franc(1).currency());
    }
}
