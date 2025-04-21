# Atomics

Concurrency Primitives

- Threads
- Locks
- Reference counts
- Atomics
- Mailboxes / Channels

- Memory Model
    - Relaxed
    - Acquire
    - Release
    - AckRel
    - SeqCst

- Operations
    - Load
    - Store
    - Compare Swap
    - Wait
    - Wake up one
    - Wake up all
    - Swap
    - Fetch
    - Add
    - Sub 

- RWLock
- RCU
    - Read copy update
- Condvar
- Fence
- Barrier
- Once
- Thread Parking
- Sequence Lock
- Compare exchange


## Resource
- https://marabos.nl/atomics 
- [Lock-Free to Wait-Free Simulation in Rust](https://www.youtube.com/watch?v=Bw8-vvtA-E8)
    - https://csaws.cs.technion.ac.il/~erez/Papers/wf-simulation-full.pdf
    - Lock based data structure
    - Fine grained locking
    - Obstruction free - None blocked
    - Lock free - One thread making progress
    - Wait free - All thread making progress
- [Lock-Free to Wait-Free Simulation in Rust (part 2)](https://www.youtube.com/watch?v=tNzCj8691LE)
- [Memory Model: Get your shared data under control - Jana Machutová - Meeting C++ 2023](https://www.youtube.com/watch?v=L5RCGDAan2Y)
- [CppCon 2015: Michael Wong “C++11/14/17 atomics and memory model..."](https://www.youtube.com/watch?v=DS2m7T6NKZQ)
- [Arvid Norberg: The C++ memory model: an intuition](https://www.youtube.com/watch?v=OyNG4qiWnmU)
- [Introduction to Wait-free Algorithms in C++ Programming - Daniel Anderson - CppCon 2024](https://youtu.be/kPh8pod0-gk)


## Library
- https://github.com/rust-lang/miri
- https://github.com/tokio-rs/loom
- 