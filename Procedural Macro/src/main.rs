// Simple macro
macro_rules! test {
    () => {
        println!("test block macro");
    };
}

// Not wokring
// Here tt stands for tokenn tree, metavariable
macro_rules! java_print {
    ( system.out.println($args:tt); ) => {
        println!("{}", $args);
    }
}

macro_rules! five_times {
    ($x:expr) => (5 * $x);
}

fn main() {
    // Simple macro uasge
    test!();
    println!("Hello, world!");

    let mut variable_name = five_times!(2 + 3);

    java_print! {
        system.out.println(variable_name);
    }
}
