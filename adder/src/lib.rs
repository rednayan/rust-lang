struct Rect {
    width : u32,
    height : u32,
}
impl Rect {
    fn can_hold(&self , other : &Rect) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub struct Guess {
    value : i32,
}

impl Guess {
    fn new(value : i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1");
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100"
            );
        }
        Guess { value  }
    }
 }



pub fn add(a:f32 , b:f32) -> f32 {
    a + b
}

pub fn gree(name:&str) -> String{
    format!("Hello {} !",name )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger : Rect = Rect { width:7, height: 8 };
        let smaller : Rect = Rect { width: 1, height: 5 };

        assert!(larger.can_hold(&smaller));
    }
    #[test] 
    fn smaller_cannot_hold_larger(){
        let larger : Rect = Rect { width: 7, height: 8 };
        let smaller : Rect = Rect { width: 1, height: 5 };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn test_for_string() {
        let result : String = gree("carol");
       assert!(result.contains("carol"), "Greetings did not contain name `{}`",result);
    }

    #[test]
    fn this_adds_two_numbers(){
        let a : f32 = 12.2;
        let b : f32 = 24.5;

        assert_eq!(36.7,add(a, b));
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn test_guess_value() {
        Guess::new(200);
    }

    #[test]
    fn it_works() -> Result<() , String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal to four"))
        }
    }
    
}
