## Learning Resources
Credit: https://github.com/crossbeam-rs/crossbeam

### Papers

* [Memory Barriers: a Hardware View for Software Hackers](http://www.rdrop.com/users/paulmck/scalability/paper/whymb.2009.04.05a.pdf)
* [Practical lock-freedom - Keir Fraser](https://www.cl.cam.ac.uk/techreports/UCAM-CL-TR-579.pdf)
* [Reclaiming Memory for Lock-Free Data Structures: There has to be a Better Way](http://www.cs.toronto.edu/~tabrown/debra/fullpaper.pdf)
* [Effective Memory Reclamation for Lock-Free Data Structures in C++](http://www.ub.tuwien.ac.at/dipl/VL/51367.pdf)
* [Comparative Performance of Memory Reclamation Strategies for Lock-free and Concurrently-readable Data Structures](http://www.cs.toronto.edu/~tomhart/papers/tomhart_thesis.pdf)
* [Lock-Free Data Structures with Hazard Pointers](https://erdani.com/publications/cuj-2004-12.pdf)
* [Lock-Free Data Structures](https://erdani.com/publications/cuj-2004-10.pdf)
* [Hazard Pointers: Safe Memory Reclamation for Lock-Free Objects](http://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.395.378&rep=rep1&type=pdf)
* [What Every Programmer Should Know About Memory](https://www.akkadia.org/drepper/cpumemory.pdf)
* [Simple, Fast, and Practical Non-Blocking and Blocking Concurrent Queue Algorithms](https://www.cs.rochester.edu/~scott/papers/1996_PODC_queues.pdf)
* [Fast and Robust Memory Reclamation for Concurrent Data Structures](https://infoscience.epfl.ch/record/218413/files/qsense-techrep.pdf)
* [Fast non-intrusive memory reclamation for highly-concurrent data structures](https://cdn.app.compendium.com/uploads/user/e7c690e8-6ff9-102a-ac6d-e4aebca50425/f4a5b21d-66fa-4885-92bf-c4e81c06d916/File/cf559034b1dbe68f39fce07ca210c422/ismm16_dice_hwhp.pdf)
* [Project Snowflake: Non-blocking safe manual memory management in .NET](https://www.microsoft.com/en-us/research/publication/project-snowflake-non-blocking-safe-manual-memory-management-net/#)
* [High Performance Transactions in Deuteronomy](https://www.microsoft.com/en-us/research/wp-content/uploads/2016/02/DeuteronomyTC-CIDR2015-full.pdf)
* [The Bw-Tree: A B-tree for New Hardware Platforms](https://www.microsoft.com/en-us/research/publication/the-bw-tree-a-b-tree-for-new-hardware/#)
* [The ART of Practical Synchronization](https://db.in.tum.de/~leis/papers/artsync.pdf)

### Articles

* [1024cores by Dmitry Vyukov](http://www.1024cores.net/)
* [Preshing on Programming](http://preshing.com/archives/)
* [Mechanical Sympathy](https://mechanical-sympathy.blogspot.com/)
* [Concurrency Freaks](http://concurrencyfreaks.blogspot.com/)
* [Concurrency in the D Programming Language](http://www.informit.com/articles/article.aspx?p=1609144&seqNum=16)
* [Lock-free Data Structures. The Inside. Memory Management Schemes](https://kukuruku.co/post/lock-free-data-structures-the-inside-memory-management-schemes/)
* [Lock-freedom without garbage collection](https://aturon.github.io/blog/2015/08/27/epoch/)
* [Linked Lists: Locking, Lock-Free, and Beyond...](http://cs.brown.edu/courses/cs176/lectures/chapter_09.pdf)
* [Locking in WebKit](https://webkit.org/blog/6161/locking-in-webkit/)
* [Lock Free Queues Index](http://psy-lob-saw.blogspot.hr/p/lock-free-queues.html)
* [Everything You Always Wanted to Know About Synchronization but Were Afraid to Ask](http://sigops.org/sosp/sosp13/papers/p33-david.pdf)
* [FAAArrayQueue - MPMC lock-free queue](http://concurrencyfreaks.blogspot.hr/2016/11/faaarrayqueue-mpmc-lock-free-queue-part.html)
* [concurrency.markmail.org](http://concurrency.markmail.org/search/?q=lock-free)
* [Designing a lock-free, wait-free hash map](https://shlomisteinberg.com/2015/09/28/designing-a-lock-free-wait-free-hash-map/)

### Videos

* Erez Petrank — Memory management for concurrent data structures
  * [Part 1](https://www.youtube.com/watch?v=aedEe0Zx_g0)
  * [Part 2](https://www.youtube.com/watch?v=BCXrG1M65HU)
  * [Part 3](https://www.youtube.com/watch?v=u9pbPpZXu18)
  * [Part 4](https://www.youtube.com/watch?v=2nc7IqfshgQ&t=1s)
* C++ and Beyond 2012: Herb Sutter - atomic<> Weapons
  * [Part 1](https://channel9.msdn.com/Shows/Going+Deep/Cpp-and-Beyond-2012-Herb-Sutter-atomic-Weapons-1-of-2)
  * [Part 2](https://channel9.msdn.com/Shows/Going+Deep/Cpp-and-Beyond-2012-Herb-Sutter-atomic-Weapons-2-of-2)
* CppCon 2014: Herb Sutter "Lock-Free Programming (or, Juggling Razor Blades)"
  * [Part 1](https://www.youtube.com/watch?v=c1gO9aB9nbs)
  * [Part 2](https://www.youtube.com/watch?v=CmxkPChOcvw)
* [Engineering Concurrent Library Components](https://www.youtube.com/watch?v=sq0MX3fHkro)
* [A Fast Wait-Free Hash Table](https://www.youtube.com/watch?v=WYXgtXWejRM)
* [Lock-free programming with modern C++ - Timur Doumler [ACCU 2017]](https://www.youtube.com/watch?v=qdrp6k4rcP4)
* [CppCon 2016: Hans Boehm “Using weakly ordered C++ atomics correctly"](https://www.youtube.com/watch?v=M15UKpNlpeM)

### Books

* C++ Concurrency in Action
* The Art of Multiprocessor Programming
* Rust Atomics and Locks

## Related Projects

* [Junction](https://github.com/preshing/junction) - Concurrent hash tables
* [libcds](https://github.com/khizmax/libcds) - Concurrent data structures
* [xenium](https://github.com/mpoeter/xenium) - A C++ library providing various concurrent data structures and reclamation schemes.
* [liburcu](liburcu.org) - Userspace RCU (read-copy-update) library
* [liblfds](https://liblfds.org/) - Lock-free data structures
* [Tervel](https://github.com/ucf-cs/Tervel) - Fast wait-free algorithms designed for shared memory systems
* [Folly](https://github.com/facebook/folly) - Facebook Open Source Library
* [Intel Thread Building Blocks](https://www.threadingbuildingblocks.org/) - A C++ template library developed by Intel for parallel programming on multi-core processors
* [libcuckoo](https://github.com/efficient/libcuckoo) - A high-performance, concurrent hash table
* [AirConcurrentMap](https://boilerbay.com/airmap/) - Java in-memory data storage component for key/value entries called a Map
* [Concurrency Kit](http://concurrencykit.org/) - Concurrency primitives, safe memory reclamation mechanisms and non-blocking data structures for the research, design and implementation of high performance concurrent systems
* [nbds](https://github.com/argv0/nbds) - C implementations of several scalable non-blocking data structures for x86 and x86-64
* [snaptree](https://github.com/nbronson/snaptree) - Concurrent TreeMap w/ efficient support for clone() and consistent iteration
* [System.Collections.Concurrent](https://github.com/mono/mono/tree/mono-3.12.1/mcs/class/corlib/System.Collections.Concurrent) - A Mono library implemented in C#
* [Scala Parallel Collections](https://github.com/scala/scala-parallel-collections/tree/master/core/src/main/scala/scala/collection/parallel) - Parallel collections standard library module for Scala 2.13+
* [Boost.Lockfree](http://www.boost.org/doc/libs/1_60_0/doc/html/lockfree.html)
* [skiptree](https://github.com/mspiegel/lockfreeskiptree) - Drop-in replacement for java.util.concurrent.ConcurrentSkipList[Map|Set]
* [Practical lock-free data structures](http://www.cl.cam.ac.uk/research/srg/netos/projects/archive/lock-free/) - By University of Cambridge
* [moodycamel::ConcurrentQueue](https://github.com/cameron314/concurrentqueue) - An industrial-strength lock-free queue for C++
* [wangziqi2013/BwTree](https://github.com/wangziqi2013/BwTree) - An open sourced implementation of Bw-Tree in SQL Server Hekaton
* [Parallel Patterns Library](https://msdn.microsoft.com/en-us/library/dd504906.aspx) - Parallel Containers and Objects
* [reagents](https://github.com/ocamllabs/reagents) - Reagents for multicore OCaml
* [ASCYLIB](https://github.com/LPD-EPFL/ASCYLIB) - Concurrent-search data-structure library with over 40 implementantions of linked lists, hash tables, skip lists, binary search trees, queues, and stacks
* [ssmem](https://github.com/LPD-EPFL/ssmem) - A simple object-based memory allocator with epoch-based garbage collection
* [palmtree](https://github.com/runshenzhu/palmtree) - An implementation of Intel's concurrent B+Tree (Palm Tree)
* [synchrobench](https://github.com/gramoli/synchrobench) - Java and C++ benchmarks for comparing many concurrent structures
