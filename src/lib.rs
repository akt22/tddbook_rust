// ==================================
// Money
// ----------------------------------
pub trait Money {
    fn amount(&self) -> i32;

    fn equals<T: Money>(&self, object: T) -> bool {
        self.amount() == object.amount()
    }
}

// ==================================
// Dollar
// ----------------------------------
#[derive(Debug, PartialEq)]
pub struct Dollar {
    amount: i32,
}

impl Money for Dollar {
    fn amount(&self) -> i32 {
        self.amount
    }
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
}

// ==================================
// Franc
// ----------------------------------
#[derive(Debug, PartialEq)]
pub struct Franc {
    amount: i32,
}

impl Franc {
    pub fn new(amount: i32) -> Franc {
        Franc { amount: amount }
    }

    pub fn times(&self, multiplier: i32) -> Franc {
        Franc {
            amount: self.amount * multiplier,
        }
    }

    pub fn equals(&self, object: Franc) -> bool {
        let franc = object;
        self.amount == franc.amount
    }
}

// ==================================
// Test
// ----------------------------------
#[cfg(test)]
mod tests {
    use super::Dollar;
    use super::Franc;
    use super::Money;

    // #[test]
    // fn test_multiplication() {
    //     assert!("hello".equals("test"));
    // }


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

    #[test]
    fn test_franc_multiplication() {
        let five = Franc::new(5);
        assert_eq!(Franc::new(10), five.times(2));
        assert_eq!(Franc::new(15), five.times(3));
    }
}
