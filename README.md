<img src="./.assets/christmas_ferris.png" width="164">

# 🎄 Advent of Code 2024

Solutions for [Advent of Code](https://adventofcode.com/) in [Rust](https://www.rust-lang.org/).
Based on this [AoC Rust template](https://github.com/fspoettel/advent-of-code-rust)

<!--- advent_readme_stars table --->
## 2024 Results

| Day | Part 1 | Part 2 |
| :---: | :---: | :---: |
| [Day 1](https://adventofcode.com/2024/day/1) | ⭐ | ⭐ |
| [Day 2](https://adventofcode.com/2024/day/2) | ⭐ | ⭐ |
| [Day 3](https://adventofcode.com/2024/day/3) | ⭐ | ⭐ |
| [Day 4](https://adventofcode.com/2024/day/4) | ⭐ | ⭐ |
| [Day 5](https://adventofcode.com/2024/day/5) | ⭐ | ⭐ |
| [Day 6](https://adventofcode.com/2024/day/6) | ⭐ | ⭐ |
| [Day 7](https://adventofcode.com/2024/day/7) | ⭐ | ⭐ |
| [Day 8](https://adventofcode.com/2024/day/8) | ⭐ | ⭐ |
| [Day 9](https://adventofcode.com/2024/day/9) | ⭐ | ⭐ |
| [Day 10](https://adventofcode.com/2024/day/10) | ⭐ | ⭐ |
| [Day 11](https://adventofcode.com/2024/day/11) | ⭐ | ⭐ |
| [Day 12](https://adventofcode.com/2024/day/12) | ⭐ | ⭐ |
| [Day 13](https://adventofcode.com/2024/day/13) | ⭐ | ⭐ |
| [Day 14](https://adventofcode.com/2024/day/14) | ⭐ | ⭐ |
| [Day 15](https://adventofcode.com/2024/day/15) | ⭐ | ⭐ |
| [Day 16](https://adventofcode.com/2024/day/16) | ⭐ | ⭐ |
| [Day 17](https://adventofcode.com/2024/day/17) | ⭐ | ⭐ |
| [Day 18](https://adventofcode.com/2024/day/18) | ⭐ | ⭐ |
| [Day 19](https://adventofcode.com/2024/day/19) | ⭐ | ⭐ |
| [Day 20](https://adventofcode.com/2024/day/20) | ⭐ | ⭐ |
| [Day 21](https://adventofcode.com/2024/day/21) | ⭐ | ⭐ |
| [Day 22](https://adventofcode.com/2024/day/22) | ⭐ | ⭐ |
| [Day 23](https://adventofcode.com/2024/day/23) | ⭐ | ⭐ |
| [Day 24](https://adventofcode.com/2024/day/24) | ⭐ | ⭐ |
| [Day 25](https://adventofcode.com/2024/day/25) | ⭐ | ⭐ |
<!--- advent_readme_stars table --->

<!--- benchmarking table --->
## Benchmarks

| Day | Part 1 | Part 2 |
| :---: | :---: | :---:  |
| [Day 1](./src/bin/01.rs) | `72.9µs` | `89.5µs` |
| [Day 2](./src/bin/02.rs) | `137.6µs` | `200.1µs` |
| [Day 3](./src/bin/03.rs) | `158.9µs` | `229.7µs` |
| [Day 4](./src/bin/04.rs) | `79.2µs` | `56.9µs` |
| [Day 5](./src/bin/05.rs) | `2.1ms` | `7.5ms` |
| [Day 6](./src/bin/06.rs) | `51.3µs` | `48.3ms` |
| [Day 7](./src/bin/07.rs) | `35.4ms` | `2.3s` |
| [Day 8](./src/bin/08.rs) | `16.3µs` | `40.7µs` |
| [Day 9](./src/bin/09.rs) | `219.4µs` | `28.8ms` |
| [Day 10](./src/bin/10.rs) | `606.4µs` | `582.1µs` |
| [Day 11](./src/bin/11.rs) | `1.1s` | `5.3ms` |
| [Day 12](./src/bin/12.rs) | `11.5ms` | `12.9ms` |
| [Day 13](./src/bin/13.rs) | `74.0µs` | `68.4µs` |
| [Day 14](./src/bin/14.rs) | `53.1µs` | `56.4ms` |
| [Day 15](./src/bin/15.rs) | `167.2µs` | `304.0µs` |
| [Day 16](./src/bin/16.rs) | `5.5ms` | `5.3ms` |
| [Day 17](./src/bin/17.rs) | `2.7µs` | `5.0µs` |
| [Day 18](./src/bin/18.rs) | `356.2µs` | `2.5ms` |
| [Day 19](./src/bin/19.rs) | `16.0ms` | `15.9ms` |
| [Day 20](./src/bin/20.rs) | `496.4µs` | `27.6ms` |
| [Day 21](./src/bin/21.rs) | `64.4µs` | `630.6µs` |
| [Day 22](./src/bin/22.rs) | `21.4ms` | `421.8ms` |
| [Day 23](./src/bin/23.rs) | `828.7µs` | `328.3ms` |
| [Day 24](./src/bin/24.rs) | `173.3µs` | `50.3ms` |
| [Day 25](./src/bin/25.rs) | `320.0µs` | `-` |

**Total: 4508.89ms**
<!--- benchmarking table --->

---

## Usage

### ➡️ Run solutions for a day

```sh
# example: `cargo solve 01`
cargo solve <day>

# output:
#     Finished dev [unoptimized + debuginfo] target(s) in 0.13s
#     Running `target/debug/01`
# Part 1: 42 (166.0ns)
# Part 2: 42 (41.0ns)
```

The `solve` command runs your solution against real puzzle inputs. To run an optimized build of your code, append the `--release` flag as with any other rust program.

### ➡️ Run all solutions

```sh
cargo all

# output:
#     Running `target/release/advent_of_code`
# ----------
# | Day 01 |
# ----------
# Part 1: 42 (19.0ns)
# Part 2: 42 (19.0ns)
# <...other days...>
# Total: 0.20ms
```

This runs all solutions sequentially and prints output to the command-line. Same as for the `solve` command, the `--release` flag runs an optimized build.

### ➡️ Benchmark your solutions

```sh
# example: `cargo time 8 --store`
cargo time <day> [--all] [--store]

# output:
# Day 08
# ------
# Part 1: 1 (39.0ns @ 10000 samples)
# Part 2: 2 (39.0ns @ 10000 samples)
#
# Total (Run): 0.00ms
#
# Stored updated benchmarks.
```

The `cargo time` command allows you to benchmark your code and store timings in the readme. When benching, the runner will run your code between `10` and `10.000` times, depending on execution time of first execution, and print the average execution time.

`cargo time` has three modes of execution:

 1. `cargo time` without arguments incrementally benches solutions that do not have been stored in the readme yet and skips the rest.
 2. `cargo time <day>` benches a single solution.
 3. `cargo time --all` benches all solutions.

By default, `cargo time` does not write to the readme. In order to do so, append the `--store` flag: `cargo time --store`.

> Please note that these are not _scientific_ benchmarks, understand them as a fun approximation. 😉 Timings, especially in the microseconds range, might change a bit between invocations.

### ➡️ Run all tests

```sh
cargo test
```
