# Rust Stream

- [Open Source Contribution](https://www.youtube.com/watch?v=8UzLuMiGs9s)

- [Rust Good Practices Talk Takeaway](https://www.youtube.com/watch?feature=youtu.be&v=Pn-1so-Ibsg&app=desktop)
    - Open source project is quite like a small start up, and trying to do just one thing is incredibly difficult.
    - Dream Job is to work full time on open source
    - Dont completely rewrite in rust.
    - A complex system that works is invariably found to have evolved from a simple system that worked. A complex system designed from scratch does not work and cannot be patched up to make it work. Yo have to start with simple system. - john gall
    - Instead of completely rewriting this system, split off tiny interfaces in new system
    - Microservice architecture to do this
    - `NPM` architecture has central registry to couchDB, which emits events and followers work on those events.
    - Dont try to change someones mind
    - `RustDoc` to generate html from documentation
    - It makes right things easy, wrong thing hard
    - `Clippy` gives good recommenndation based on static analysis.
    - Rust log parser was slower than node log parser. Writing naive things makes it slow.
    - You are never dedicated to something you have complete confidence in. If you are arguing about something this dogma must be in doubt.
    - `Glue`, `Nom`, `Regex` for parsing
    - Writing boring code when company code is at stake is good
    - Trying to change everything at once is a recipy for failure
    - Have a problem to solve, not what you want to solve.
    - Just replace the problematic part in rust

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