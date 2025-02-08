extern crate num_cpus;

pub fn print_cpu() {
    let num = num_cpus::get();
    println!("{}", "cpu test");
    println!("{}", num);
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
