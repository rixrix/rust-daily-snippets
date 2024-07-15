use std::fmt::{Display, Error};

struct CreditCardNumber(String);

impl Display for CreditCardNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), Error> {
        write!(f, "{} **** **** {}", &self.0[..4], &self.0[15..])
    }
}

fn main() {
    let raw = String::from("1111 2222 3333 4444");
    println!("Credit Card Information: {} ", CreditCardNumber(raw));
}