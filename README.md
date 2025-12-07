# 🎄 Advent Of Code 2025

![rust logo][rust-badge] ![veryl logo][veryl-badge]

My solutions for the [Advent of Code 2025][advent-of-code] challenges.

As a learning exercise, I'm attempting to implement some solutions in hardware using a relatively recent Hardware Description Language (HDL) called [Veryl][veryl] alongside my Rust solutions!

I'm interested in exploring how well certain problems can be mapped into a simple hardware design (that could theoretically be synthesised for an FPGA) using counters, FSMs, combinatorial logic and maybe a multi-stage pipeline (i.e. decoder → solver), and comparing this to a "more optimised" software solution (e.g. using bitwise operations, modular arithmetic, range intervals, etc.).

Certain types of problems may lend themselves well to hardware implementation by taking advantage of parallelism (e.g. day 2, repeating patterns could be detected in a single cycle), while others may be inherently sequential with complex mathematical operations that a CPU is designed to handle very efficiently.

The hardware implementation is designed to solve the **second part** of each day's challenge. The benchmark uses the same full input as the software solution. Each hardware solution is structured as a simple streaming pipeline that processes the input data one byte at a time, applying backpressure when necessary to handle variable processing times. The simulation time (from first byte to correct answer) is **simulated at 1 GHz** to give an idea of how fast the design _could maybe run_ on real hardware.

<div align="center">

|      Day | Name                |   Rust    | Opt. |   Veryl   | Rust (p1) | Rust (p2) | Veryl (1 GHz) | Speed-up |
| -------: | ------------------- | :-------: | :--: | :-------: | --------: | --------: | ------------: | -------: |
| [1][p01] | Secret Entrance     | [01][s01] |  ⚡  | [01][v01] |     33 µs |     39 µs |         21 µs |      1.9 |
| [2][p02] | Gift Shop           | [02][s02] |  ⚡  | [02][v02] |    192 µs |    7.6 ms |       2.09 ms |      3.6 |
| [3][p03] | Lobby               | [03][s03] |  ⚡  | [03][v03] |    5.4 µs |     23 µs |         20 µs |      1.2 |
| [4][p04] | Printing Department | [04][s04] |  ⚡  | [04][v04] |    238 µs |    279 µs |         19 µs |       15 |
| [5][p05] | Cafeteria           | [05][s05] |  ⚡  | [05][v05] |     57 µs |    7.3 µs |       5.8 µs¹ |      1.3 |
| [6][p06] | Trash Compactor     | [06][s06] |  ⚡  | [06][v06] |     14 µs |     13 µs |         22 µs |      0.6 |
| [7][p07] | Laboratories        | [07][s07] |  ⚡  | [07][v07] |    8.0 µs |    8.0 µs |         20 µs |      0.4 |
|          | ...                 |           |      |           |           |           |               |          |

**Key:** &nbsp;&nbsp; 🌱 It works &nbsp;&nbsp; ⚡ Optimised

_Rust benchmarked on an AMD Ryzen 9 9950X (5.7 GHz)_

</div>

<sub>
<i>¹ Idle cycles (due to slow input streaming of irrelevant/unused problem data at 1 B/cycle) were subtracted from the total simulation time to give a more accurate measure of the hardware processing speed.</i>
</sub>

## Usage

Install [Rust](https://www.rust-lang.org/).

Run the following commands to run the project:

```sh
# Run all tests
cargo test

# Solve a specific day
cargo solve <day> --release

# Solve all days in release mode
cargo all --release

# Benchmark a solution in release mode
cargo time <day>
```

Tests can be run without any extra configuration as they use the AoC examples, which are committed
in this repository under `data/`. If you want to run the solutions against your own input data, place them into the `/data/inputs/` (e.g. `01.txt` for day 1) and run `cargo solve`. Benchmarks were run on my personal problem inputs.

## Acknowledgments

This repository uses a modified version of [this template][template]. Thanks Felix!

## License

Distributed under the MIT Licence. See [LICENCE](LICENCE) for more information.

[rust-badge]: https://img.shields.io/badge/Rust-d55826?logo=rust&style=for-the-badge
[veryl-badge]: https://img.shields.io/badge/Veryl-b7b8f3?style=for-the-badge
[advent-of-code]: https://adventofcode.com/
[rust]: https://www.rust-lang.org/
[veryl]: https://veryl-lang.org/
[template]: https://github.com/fspoettel/advent-of-code-rust
[p01]: https://adventofcode.com/2023/day/1
[p02]: https://adventofcode.com/2023/day/2
[p03]: https://adventofcode.com/2023/day/3
[p04]: https://adventofcode.com/2023/day/4
[p05]: https://adventofcode.com/2023/day/5
[p06]: https://adventofcode.com/2023/day/6
[p07]: https://adventofcode.com/2023/day/7
[s01]: src/bin/01.rs
[s02]: src/bin/02.rs
[s03]: src/bin/03.rs
[s04]: src/bin/04.rs
[s05]: src/bin/05.rs
[s06]: src/bin/06.rs
[s07]: src/bin/07.rs
[v01]: hardware/01.veryl
[v02]: hardware/02.veryl
[v03]: hardware/03.veryl
[v04]: hardware/04.veryl
[v05]: hardware/05.veryl
[v06]: hardware/06.veryl
[v07]: hardware/07.veryl
