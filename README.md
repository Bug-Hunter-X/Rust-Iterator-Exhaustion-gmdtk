# Rust Iterator Exhaustion Bug

This repository demonstrates a common error in Rust: iterator exhaustion. The `bug.rs` file contains code that iterates over a vector using an iterator.  After consuming all elements, attempting to access further elements leads to a panic. The corrected code in `bugSolution.rs` shows the use of cloning the iterator to enable multiple iterations.