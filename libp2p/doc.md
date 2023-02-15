# libp2p

- Ethereum concensus and execution is different
- Addressing
    - MultiAddr
- Peer ID
- Well Explained
    - https://www.parity.io/blog/why-libp2p

Playlist
https://youtu.be/1WuXJ9zBAqw?list=PLhuBigpl7lqs-S5hlJm44a0KGtDRc4-O2
- Later

Filecoin Concepts
- Based on orderbook
- Uses `bitswap`
- QUIC ( Transport + Encryption + Multiplexing ) 
    - Based on UDP
    - TCP has many unencrypted transport information

- https://youtu.be/fyhZWlDbcyM
    - Nat hole punching with relay

- [exploring rust-libp2p - a p2p implementation in Rust](https://youtu.be/bz4Y_HwyEqM)

- libp2p rust walkthrough
    - Transport interface defined
    - Connection takes in a stream, writes message to a stream, reads into a buffer
    - `std::pin`, `std::Box`