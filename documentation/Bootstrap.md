# Bootstrap

```shell
cargo new project --lib

# lib.rs
pub fn print_cpu() {
    println!("{}", "cpu test");
}

# main.rs
use cpu::print_cpu;

fn main() {
    print_cpu()
}

cargo run
```