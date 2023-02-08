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