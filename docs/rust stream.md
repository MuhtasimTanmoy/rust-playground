# Rust Stream

- [Rust Good Practices Talk Takeaway](https://www.youtube.com/watch?feature=youtu.be&v=Pn-1so-Ibsg&app=desktop)
    - Open source projects share similarities with small startups, and attempting to focus on just one thing can prove difficult.
    - For some, working full-time on open source is a dream job.
    - A complete rewrite in Rust may not always be necessary.
    - According to John Gall's philosophy, complex systems that work evolved from simple systems that worked, whereas complex systems designed from scratch often fail and cannot be fixed.
    - Rather than doing a complete rewrite, consider breaking down small interfaces into a new system using microservice architecture.
    - For example, the NPM architecture has a central registry that emits events, and followers work on those events.
    - Use tools such as `RustDoc` to generate HTML from documentation and `Clippy` to provide recommendations based on static analysis.
    - The Rust philosophy is to make the right things easy and the wrong things hard.
    - `Clippy` provides helpful recommendations based on static analysis.
    - Writing naive code can result in slow performance, as seen with the Rust `log parser` being slower than the node log parser.
    - Only dedicate yourself to something you have complete confidence in. If there is any doubt or argument, the dogma must be in question.
    - Use tools such as `Glue, Nom, and Regex` for parsing.
    - Writing boring code when company code is at stake is good.
    - Trying to change everything at once is a recipe for failure.
    - Focus on solving the problem at hand, not what you want to solve.
    - Consider replacing problematic parts with Rust.

- [Rust at Speed](https://www.youtube.com/watch?v=s19G6n0UjsM&t=3s) 
    - Explains usage of rust on [Noria](https://github.com/mit-pdos/noria)
    - It is a database that uses a cache for materialized views, which are the current results for a query.
    - The problem with concurrent read and write operations on a large result table and partial materialized view can lead to performance issues.
    - Using `locks and RWLock` can be too costly due to the additional wrapping work.
    - Noria uses two maps, one for reading and one for writing, to address this issue.
    - An epic counter increment of +2 is maintained for each read, which is used to switch between the two maps.

- [Channels](https://www.youtube.com/watch?v=b4mS5UPHh20)
- [Open Source Contribution](https://www.youtube.com/watch?v=8UzLuMiGs9s)
- [Crust of Rust: Subtyping and Variance](https://www.youtube.com/watch?v=iVYWDIW71jk)
    - Explains covariance, invariance, contra variance

- [Just Rust](https://www.youtube.com/watch?v=YtUfK3ZP3No&list=PLFCH6yhq9yAH28S_oGUtqO46eI7IAWdEO)
    - `Valgrind` can find memory error, race condition
    - `Covety` static analysis, defect finding tool

- [The Why, What, and How of Pinning in Rust](https://youtu.be/DkMwYxfSYNQ)
    - auto traits are `send`, `sync`, `unpin`
    - Pin will put constraint on the item to not be `movable`
    - future highly uses it

- [Boats on Pinning](https://youtu.be/shtfSMTwKRw)
    - Self referenctial `struct` handle
    - Once you start polling a future you cant move it again
    - As all `future` on await makes a `state machine`

- Rust Course From Ardan Labs
    - Not exception rather error check has good performance
    - Cargo.toml
        - opt-level = 1
        - overflow-checks = false
    - Link time optimization (`lto` flag)
        - none
        - thin
            - `compile` time slow, optimized 
        - fat
    - Only spend time optimizing that which is needed
    - Benchmark
        - Use `criterion`
    - Memory size optimize