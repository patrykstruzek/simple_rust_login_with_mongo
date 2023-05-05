pub mod input {
    use std::io;
    use crate::user::User;

    impl User {
        fn login_name(&self) {
            match io::stdin().read_line(&mut self.name) {
                Ok(_) => (),
                Err(e) => println!("Can not read an input! {}", e)
            };
        }
    
        fn email_input(&self) {
            match io::stdin().read_line(&mut self.email) {
                Ok(_) => (),
                Err(e) => println!("Can not read an input! {}", e)
            }
        }
    
        fn password_input(&self) {
            match io::stdin().read_line(&mut self.password) {
                Ok(_) => (),
                Err(e) => println!("Can not read an input! {}", e)
            }
        }
    }

}