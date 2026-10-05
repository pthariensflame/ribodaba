# ribodaba

“**Ri**ng-**B**uffer-**O**ptimized **D**e-**A**mortized **B**anker’s **A**ggregator”: a Rust
implementation of the algorithm from [Tangwongsan, Hirzel, and Schneider
(2017)](https://doi.org/10.1145/3093742.3093925), modified to run atop `std::collections::VecDeque` (a
ring buffer) instead of the original linked-list-of-chunks implementation.

This crate exposes the core DABA data structure as well as an iterator adaptor that uses it to implement
sliding window aggregation.  This crate is `no_std`-compatible, but does require `alloc`.
