# Threads

Topic to study
- Threads
- Interior mutability
- Send
    - Marker trait, which have no method
    - This trait indicates that it is okay to send this type to another thread
    - RC( non atomic referene counted type ) and MutexGuard, RWLock dont have Send, as OS specific constraint
- Sync
- Mutex
- Condition variable
- Thread parking


- Marker Trait
    - Compiler will autometically implement this trait for you, if all members are themselves same trait

## Reference
- [Crust of Rust: Send, Sync, and their implementors](https://youtu.be/yOezcP-XaIw)