//maximum number to select from
pub const MAX_NUM: i32 = 256;

//create a new type and implement validations
pub struct Guess {
    value: i32,
}

impl Guess {
    ///Guarantees 1 <= `value` <= MAX_NUM.
    pub fn new(value: i32) -> Guess {
        //panic! to alert programmer who is writing calling code there is a bug
        if value < 1 || value > MAX_NUM {
            panic!("Guess value must be between 1 and {}, got {}.", MAX_NUM, value);
        }

        Guess {
            value
        }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    #[should_panic]
    fn test_new_invalid() {
        let _g = Guess::new(-10);
        let _g = Guess::new(0);
        let _g = Guess::new(MAX_NUM+1);
    }

    #[test]
    fn test_new_valid() {
        let g = Guess::new(1);
        assert_eq!(g.value(), 1);
        let g = Guess::new(MAX_NUM-1);
        assert_eq!(g.value(), MAX_NUM-1);
        let g = Guess::new(MAX_NUM);
        assert_eq!(g.value(), MAX_NUM);
    }
}