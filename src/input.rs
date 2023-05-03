use user;
use std::io;
extern crate regex;
use regex;

pub mod input {
    impl Input for User {
        fn login_name(&self) {
            match io::stdin().read_line(&mut self.name) {
                Ok(i) => i,
                Err(e) => Err(String::from("Can not read an input! {}, e"))
            }
        }
    
        fn email_input(&self) {
            match io::stdin().read_line(&mut self.email) {
                Ok(i) => i,
                Err(e) => Err(String::from("Can not read an input! {}, e"))
            }
        }
    
        fn password_input(&self) {
            match io::stdin().read_line(&mut self.password) {
                Ok(i) => i,
                Err(e) => Err(String::from("Can not read an input! {}, e"))
            }
        }
    }

}