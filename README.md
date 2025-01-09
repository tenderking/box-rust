# Box Stacking Optimization in Rust

## Project Overview

Inspired by playing with my kid and stacking boxes, I wanted to find an optimal way to stack ten boxes into three towers of roughly equal height. I measured each box and decided to tackle this problem using Rust.

## Box Heights

Here are the heights of the boxes:

```rust
let box_values = HashMap::from([
    ("box1", 4.0),
    ("box2", 4.9),
    ("box3", 6.0),
    ("box4", 7.2),
    ("box5", 8.3),
    ("box6", 9.5),
    ("box7", 10.5),
    ("box8", 11.3),
    ("box9", 12.5),
    ("box10", 13.6),
]);
```
Results

The algorithm identifies valid combinations of box stacks with heights as close as possible. Here's how to run the code and see the results.
How to Run

    Make sure you have Rust and Cargo installed. If not, follow the instructions at https://www.rust-lang.org/tools/install

    Clone the Repository:
    Bash

git clone <repository-url>
cd <repository-directory>

Build and Run:
Bash

    cargo run

Conclusion

This project demonstrates using Rust to solve a practical box-stacking optimization problem. The algorithm finds combinations of stacks with nearly equal heights, offering a fun and educational way to explore Rust's capabilities.

