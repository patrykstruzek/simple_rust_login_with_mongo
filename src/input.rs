pub mod input {
    use crate::user::User;
    use regex::Regex;
    use std::io;

    impl User {
        pub fn name_input(&mut self) -> String {
            match io::stdin().read_line(&mut self.name) {
                Ok(_) => (),
                Err(e) => println!("Can not read an input! {}", e),
            };
            return self.name.trim().to_string();
        }

        pub fn email_input(&mut self) -> String {
            match io::stdin().read_line(&mut self.email) {
                Ok(_) => (),
                Err(e) => println!("Can not read an input! {}", e),
            }
            return self.email.trim().to_string();
        }

        fn check_passwd_regex(passwd: String) -> bool {
            let regex = Regex::new(r"^(?=.*[A-Z])(?=.*[a-z])(?=.*[!-])[A-Za-z!-]{8,}$").unwrap();
            if regex.is_match(&passwd.trim()) {
                false
            } else {
                println!(
                    "The password should contain at least one uppercase letter,
                    one lowercase letter, one special character and be at least 8 characters long.
                    Please try again!"
                );
                true
            }
        }

        pub fn password_input(&mut self) -> String {
            loop {
                match io::stdin().read_line(&mut self.password) {
                    Ok(_) => {
                        if !Self::check_passwd_regex(self.password.clone()) {
                            break;
                        }
                    }
                    Err(e) => println!("Can not read an input! {}", e),
                }
            }
            return self.password.trim().to_string();
        }
    }
}
