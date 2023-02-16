# Rust Playground

> This repository offers resources for studying Rust, including:

- [Channel](./channel/)
- [CPU](./cpu/)
- [pow-blockcahin](./pow-blockchain/)
- [procedural macro](./procedural%20macro/)
- [static dynamic linking](./static-dynamic-linking)
- [XDP-BPF]
- [TCP Transport Layer](./tcp-transport)
- [Merkle Tree](./merkle%20tree/)
- [Libp2p](./libp2p)
- [Zero Knowledge](./zero-knowledge)

The repository also includes sample code and code snippets that demonstrate the concepts. See [documentation](./documentation/) for learning materials.


## RESOURCES:
- [Rust ownership, safety explained](https://words.steveklabnik.com/a-30-minute-introduction-to-rust)
- [Macro Hygienic](https://en.wikipedia.org/wiki/Hygienic_macro)
- [How Rust ownership works?](https://static.rust-lang.org/doc/master/book/ownership.html)
- [Error Handling Rust Documentation](https://static.rust-lang.org/doc/master/book/error-handling.html)
- [Sample Library Usage](https://github.com/brson/stdx/blob/master/README.md)

## TALKS

- [Rust at Speed](https://www.youtube.com/watch?v=s19G6n0UjsM&t=3s) 
    - Expalains usage of rust on [Noria](https://github.com/mit-pdos/noria)
    - Usage of cache inside DB, mainly materialized view, the current result for a query.
    - Problem: Huge result table, concurrent read write on same table, partial materialized view.
    - Lock, RWLock fails being the costly one themselves as the wrapping work is too less.
    - Maintain two maps, one for read, another for write maintaining epic counter +2 for each read and  switch for alternatively.

- [Crust of Rust: Subtyping and Variance](https://www.youtube.com/watch?v=iVYWDIW71jk)
    - Explains covariance, invariance, contra variance

- [Crust of Rust: Channels](https://www.youtube.com/watch?v=b4mS5UPHh20)

## Docs
- [The Little Book of Rust Books](https://lborb.github.io/book/)
- [Rust verification tools 2021](https://alastairreid.github.io/automatic-rust-verification-tools-2021/)
- [Periodic Table of Rust Types](http://cosmic.mearie.org/2014/01/periodic-table-of-rust-types)
- [Rust String Conversions Cheat Sheet](https://docs.google.com/spreadsheets/d/19vSPL6z2d50JlyzwxariaYD6EU2QQUQqIDOGbiGQC7Y/pubhtml?gid=0&single=true)
- [Rust Iterator Cheat Sheet](https://danielkeep.github.io/itercheat_baked.html)
- [Rust Container Cheet Sheet](https://docs.google.com/presentation/d/1q-c7UAyrUlM-eZyTo1pd8SZ0qwA_wYxmPZVOQkoDmH4/edit)
- [Rust API Guidelines](https://github.com/brson/rust-api-guidelines)
- https://caniuse.rs/
- https://github.com/rust-unofficial/awesome-rust/
- https://lib.rs
- https://cheats.rs/
- https://github.com/rust-secure-code/projects
- https://github.com/pretzelhammer/rust-blog/blob/master/posts/tour-of-rusts-standard-library-traits.md
- [Another Container Cheet Sheet](https://i.redd.it/220xo2f6wci51.png)
- [Rust Documentation](https://static.rust-lang.org/doc/master/book/getting-started.html)