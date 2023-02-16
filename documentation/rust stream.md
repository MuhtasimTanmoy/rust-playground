# Rust Stream

- [Open Source Contribution](https://www.youtube.com/watch?v=8UzLuMiGs9s)

- [Rust Good Practices Talk Takeaway](https://www.youtube.com/watch?feature=youtu.be&v=Pn-1so-Ibsg&app=desktop)
    - Open source projects are similar to small startups and attempting to do just one thing can prove difficult.
    - A dream job would be to work full-time on open source.
    - A complete rewrite in Rust is not always necessary.
    - John Gall's philosophy states that a complex system that works evolved from a simple system that worked, while a complex system designed from scratch often fails and cannot be fixed.
    - Instead of a complete rewrite, split off small interfaces into a new system using microservice architecture.
    - The NPM architecture, for example, has a central registry that emits events and followers work on those events.
    - Don't try to change someone's mind.
    - Use tools like RustDoc to generate HTML from documentation and Clippy to provide recommendations based on static analysis.
    - It makes right things easy, wrong things hard
    - `Clippy` gives good recommendations based on static analysis.
    - Writing naive code can result in slow performance, as seen with the Rust log parser being slower than the node log parser.
    - Only be dedicated to something you have complete confidence in. If there's any doubt or argument, the dogma must be in question.
    - Use tools like `Glue, Nom, and Regex` for parsing.
    - Writing boring code when company code is at stake is good.
    - Trying to change everything at once is a recipe for failure.
    - Focus on solving the problem at hand, not what you want to solve.
    - Replace the problematic part with Rust.

- [Rust at Speed](https://www.youtube.com/watch?v=s19G6n0UjsM&t=3s) 
    - Expalains usage of rust on [Noria](https://github.com/mit-pdos/noria)
    - Usage of cache inside DB, mainly materialized view, the current result for a query.
    - Problem: Huge result table, concurrent read write on same table, partial materialized view.
    - Lock, RWLock fails being the costly one themselves as the wrapping work is too less.
    - Maintain two maps, one for read, another for write maintaining epic counter +2 for each read and  switch for alternatively.

- [Crust of Rust: Subtyping and Variance](https://www.youtube.com/watch?v=iVYWDIW71jk)
    - Explains covariance, invariance, contra variance

- [Crust of Rust: Channels](https://www.youtube.com/watch?v=b4mS5UPHh20)

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
    - benchmark 
        - Use `criterion`
    - memory size optimize