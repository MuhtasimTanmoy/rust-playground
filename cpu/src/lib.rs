
pub fn print_cpu() {
    println!("{}", "cpu test");
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
