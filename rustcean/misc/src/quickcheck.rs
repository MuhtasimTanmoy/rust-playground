fn reverse<T: Clone>(xs: &[T]) -> Vec<T> {
    let mut rev = vec![];
    for x in xs.iter() {
        rev.insert(0, x.clone())
    }
    rev
}

#[cfg(test)]
mod tests {
    use super::reverse;
    use quickcheck::quickcheck;

    quickcheck! {
          fn prop(xs: Vec<u32>) -> bool {
              xs == reverse(&reverse(&xs))
          }
    }
}
