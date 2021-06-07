pub struct Test { }

impl Test {
    pub fn connect() {
        println!("{} days", 31);
    }
}

pub fn test() {
    println!("Test");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        Test::connect();
    }
}
