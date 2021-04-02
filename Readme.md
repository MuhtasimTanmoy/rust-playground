# Rust Playground

This repository contains rust learning materials, sample codebase, code snippets containing concept implementation.


RESOURCES:

- [Rust ownership, safety explained](https://words.steveklabnik.com/a-30-minute-introduction-to-rust)

- [Rust Documentation](https://static.rust-lang.org/doc/master/book/getting-started.html)

- [Macro Hygienic](https://en.wikipedia.org/wiki/Hygienic_macro)

- [How Rust ownership works?](https://static.rust-lang.org/doc/master/book/ownership.html)

- [Error Handling RustDoc](https://static.rust-lang.org/doc/master/book/error-handling.html)

- [Sample Library Usage](https://github.com/brson/stdx/blob/master/README.md)

TALKS : 
- [Rust at Speed](https://www.youtube.com/watch?v=s19G6n0UjsM&t=3s) 
    - Expalains usage of rust on [Noria](https://github.com/mit-pdos/noria)
    - Usage of cache inside DB, mainly materialized view, the current result for a query.
    - Problem : Huge result table, concurrent read write on same table, partial materialized view.
    - Lock, RWLock fails being the costly one themselves as the wrapping work is too less
    - Maintain two maps, one for read, another for write maintaining epic counter +2 for each read and  switch for alternatively.

- [Crust of Rust: Subtyping and Variance](https://www.youtube.com/watch?v=iVYWDIW71jk)
    - Explains covariance, invariance, contra variance

- [Crust of Rust: Channels](https://www.youtube.com/watch?v=b4mS5UPHh20)
    -