# Advent of Code 2020

Yet another year with [Advent of Code](https://adventofcode.com/). Will probably
not finish this year either but will try to do it in Rust to improve my skills.

## Test

```sh
# Test all days so far.
% cargo test

# Example
% cargo test
    Finished test [unoptimized + debuginfo] target(s) in 0.00s
     Running target/debug/deps/advent_of_code_2020-289eca52c0997991

running 2 tests
test solutions::day01::tests::part_one ... ok
test solutions::day01::tests::part_two ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## Execute

```sh
# Run a specific day
% carog run <day: i32>

# Example
% cargo run 1
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/advent-of-code-2020 1`
Solution part 1: 197451
Solution part 2: 138233720
```
