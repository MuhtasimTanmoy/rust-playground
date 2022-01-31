# Channel

Data structures needed

- Arc
    - Reference counter to address same object queue in sender, receiver.

- Condvar
    - When it is waiting for it to be awakened it can wake up for unknown reasons by processor.
    - Wait gives up the lock just before it goes to sleep.
    - After wait is finished, the mutex is given back.
     
- Mutex
    - Similar to binary semaphore with kernel level optimization.
    - Should be given on both sender receiver.