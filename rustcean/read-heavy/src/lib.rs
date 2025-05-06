pub fn strlen(s: impl AsRef<str>) -> usize {
    s.as_ref().len()
}

pub fn strlength<S>(s: S) where S: AsRef<str> {
    s.as_ref().len()
}

fn foo() {
    strlen("test");
    strlen(String::from("test"));
}