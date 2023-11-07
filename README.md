# rc4ok
Lightweight High-Performance Cryptographically Strong Random Number Generator based on Improved RC4

## Overview

RC4OK is a light-weight, high-performance, cryptographically secure random number generator, which is based on an improved version of the RC4 stream cipher, which was proposed in https://ia.cr/2023/1486. It's suitable for use in IOT devices, which might lack presence of an operating system managed pseudo random number generator.

RC4OK pseudo-random number generator can be initialized with a non-empty seed string and it should produce arbitrary long pseudo-random bytes. *True* random events such as external peripheral interrupts can be used as entropy source and they can be added to the RC4OK PRNG state, *though not yet in a thread-safe manner, in this implementation*.

## Prerequisites
Rust stable toolchain; see https://rustup.rs for installation guide.

```bash
# When developing this library, I was using
$ rustc --version
rustc 1.73.0 (cc66ad468 2023-10-03)
```

I advise you to also use `cargo-criterion` for running benchmark executable. Read more about it @ https://crates.io/crates/cargo-criterion. You can just issue following command for installing it system-wide.

```bash
cargo install cargo-criterion
```

## Testing

For ensuring functional correctness and conformance of this RC4OK PRNG implementation, I generate Known Answer Tests using the official implementation by the RC4OK authors, living @ https://github.com/emercoin/rc4ok.

> **Note**
Those (reproducible) steps for generating KAT files are described in the gist https://gist.github.com/itzmeanjan/5d1379b4d324e888a2683d2820b57e23.

Issue following command to run all test cases.

```bash
cargo test --lib
```

## Benchmarking

Issue following command for benchmarking RC4OK PRNG, with variable length input and output.

> **Warning**
When benchmarking make sure you've disabled CPU frequency scaling, otherwise numbers you see can be pretty misleading. I found https://github.com/google/benchmark/blob/b40db869/docs/reducing_variance.md helpful.

```bash
# In case you didn't install `cargo-criterion`, you've to run benchmark with
# RUSTFLAGS="-C opt-level=3 -C target-cpu=native" cargo bench rc4ok

RUSTFLAGS="-C opt-level=3 -C target-cpu=native" cargo criterion rc4ok
```

### On *12th Gen Intel(R) Core(TM) i7-1260P*

```bash
rc4ok/8B key/32B out (cached)                                                                             
                        time:   [2574.0575 cycles 2583.6185 cycles 2593.2527 cycles]
                        thrpt:  [64.8313 cpb 64.5905 cpb 64.3514 cpb]
rc4ok/8B key/32B out (random)                                                                             
                        time:   [2606.7636 cycles 2617.6775 cycles 2628.2137 cycles]
                        thrpt:  [65.7053 cpb 65.4419 cpb 65.1691 cpb]

rc4ok/8B key/128B out (cached)                                                                             
                        time:   [2997.8248 cycles 3004.5399 cycles 3012.1415 cycles]
                        thrpt:  [22.1481 cpb 22.0922 cpb 22.0428 cpb]
rc4ok/8B key/128B out (random)                                                                             
                        time:   [3043.8787 cycles 3055.6564 cycles 3066.6192 cycles]
                        thrpt:  [22.5487 cpb 22.4681 cpb 22.3815 cpb]

rc4ok/8B key/512B out (cached)                                                                             
                        time:   [4658.6544 cycles 4674.6998 cycles 4694.8666 cycles]
                        thrpt:  [9.0286 cpb 8.9898 cpb 8.9590 cpb]
rc4ok/8B key/512B out (random)                                                                             
                        time:   [4694.0311 cycles 4703.3597 cycles 4713.2270 cycles]
                        thrpt:  [9.0639 cpb 9.0449 cpb 9.0270 cpb]

rc4ok/8B key/2048B out (cached)                                                                             
                        time:   [11207.3971 cycles 11216.4134 cycles 11224.6109 cycles]
                        thrpt:  [5.4594 cpb 5.4555 cpb 5.4511 cpb]
rc4ok/8B key/2048B out (random)                                                                             
                        time:   [11259.0064 cycles 11270.4428 cycles 11281.7112 cycles]
                        thrpt:  [5.4872 cpb 5.4817 cpb 5.4762 cpb]

rc4ok/8B key/8192B out (cached)                                                                             
                        time:   [37420.0994 cycles 37435.8935 cycles 37451.9318 cycles]
                        thrpt:  [4.5673 cpb 4.5654 cpb 4.5634 cpb]
rc4ok/8B key/8192B out (random)                                                                             
                        time:   [37585.0076 cycles 37608.9629 cycles 37637.5633 cycles]
                        thrpt:  [4.5899 cpb 4.5865 cpb 4.5835 cpb]

rc4ok/32B key/32B out (cached)                                                                             
                        time:   [2572.5247 cycles 2578.7168 cycles 2584.8764 cycles]
                        thrpt:  [40.3887 cpb 40.2924 cpb 40.1957 cpb]
rc4ok/32B key/32B out (random)                                                                             
                        time:   [2615.5792 cycles 2625.2851 cycles 2635.2579 cycles]
                        thrpt:  [41.1759 cpb 41.0201 cpb 40.8684 cpb]

rc4ok/32B key/128B out (cached)                                                                             
                        time:   [2989.8377 cycles 2997.4252 cycles 3004.3600 cycles]
                        thrpt:  [18.7773 cpb 18.7339 cpb 18.6865 cpb]
rc4ok/32B key/128B out (random)                                                                             
                        time:   [3036.4210 cycles 3044.4885 cycles 3052.7608 cycles]
                        thrpt:  [19.0798 cpb 19.0281 cpb 18.9776 cpb]

rc4ok/32B key/512B out (cached)                                                                             
                        time:   [4652.0364 cycles 4660.6068 cycles 4668.8399 cycles]
                        thrpt:  [8.5824 cpb 8.5673 cpb 8.5515 cpb]
rc4ok/32B key/512B out (random)                                                                             
                        time:   [4678.9572 cycles 4692.8719 cycles 4705.5602 cycles]
                        thrpt:  [8.6499 cpb 8.6266 cpb 8.6010 cpb]

rc4ok/32B key/2048B out (cached)                                                                             
                        time:   [11214.7297 cycles 11225.2264 cycles 11234.2678 cycles]
                        thrpt:  [5.4011 cpb 5.3967 cpb 5.3917 cpb]
rc4ok/32B key/2048B out (random)                                                                             
                        time:   [11266.6827 cycles 11279.7609 cycles 11293.4054 cycles]
                        thrpt:  [5.4295 cpb 5.4230 cpb 5.4167 cpb]

rc4ok/32B key/8192B out (cached)                                                                             
                        time:   [37444.1838 cycles 37474.6635 cycles 37511.3311 cycles]
                        thrpt:  [4.5612 cpb 4.5567 cpb 4.5530 cpb]
rc4ok/32B key/8192B out (random)                                                                             
                        time:   [37568.3293 cycles 37599.5151 cycles 37636.0638 cycles]
                        thrpt:  [4.5764 cpb 4.5719 cpb 4.5681 cpb]

rc4ok/128B key/32B out (cached)                                                                             
                        time:   [2571.3882 cycles 2579.3929 cycles 2587.1145 cycles]
                        thrpt:  [16.1695 cpb 16.1212 cpb 16.0712 cpb]
rc4ok/128B key/32B out (random)                                                                             
                        time:   [2623.1407 cycles 2631.9951 cycles 2641.2559 cycles]
                        thrpt:  [16.5078 cpb 16.4500 cpb 16.3946 cpb]

rc4ok/128B key/128B out (cached)                                                                             
                        time:   [3004.6187 cycles 3012.6715 cycles 3020.3793 cycles]
                        thrpt:  [11.7984 cpb 11.7682 cpb 11.7368 cpb]
rc4ok/128B key/128B out (random)                                                                             
                        time:   [3049.2580 cycles 3062.2966 cycles 3074.7457 cycles]
                        thrpt:  [12.0107 cpb 11.9621 cpb 11.9112 cpb]

rc4ok/128B key/512B out (cached)                                                                             
                        time:   [4651.8726 cycles 4659.8927 cycles 4668.0468 cycles]
                        thrpt:  [7.2938 cpb 7.2811 cpb 7.2686 cpb]
rc4ok/128B key/512B out (random)                                                                             
                        time:   [4710.3644 cycles 4718.4407 cycles 4726.6339 cycles]
                        thrpt:  [7.3854 cpb 7.3726 cpb 7.3599 cpb]

rc4ok/128B key/2048B out (cached)                                                                             
                        time:   [11224.8340 cycles 11233.1534 cycles 11241.3467 cycles]
                        thrpt:  [5.1661 cpb 5.1623 cpb 5.1585 cpb]
rc4ok/128B key/2048B out (random)                                                                             
                        time:   [11273.2337 cycles 11284.8074 cycles 11296.7530 cycles]
                        thrpt:  [5.1915 cpb 5.1860 cpb 5.1807 cpb]

rc4ok/128B key/8192B out (cached)                                                                             
                        time:   [37452.5475 cycles 37471.4525 cycles 37493.1134 cycles]
                        thrpt:  [4.5064 cpb 4.5038 cpb 4.5015 cpb]
rc4ok/128B key/8192B out (random)                                                                             
                        time:   [37517.3849 cycles 37533.3325 cycles 37551.2108 cycles]
                        thrpt:  [4.5134 cpb 4.5112 cpb 4.5093 cpb]

rc4ok/512B key/32B out (cached)                                                                             
                        time:   [3787.2402 cycles 3800.8097 cycles 3812.1764 cycles]
                        thrpt:  [7.0077 cpb 6.9868 cpb 6.9618 cpb]
rc4ok/512B key/32B out (random)                                                                             
                        time:   [3856.9078 cycles 3864.3158 cycles 3871.6352 cycles]
                        thrpt:  [7.1170 cpb 7.1035 cpb 7.0899 cpb]

rc4ok/512B key/128B out (cached)                                                                             
                        time:   [4234.2484 cycles 4241.7623 cycles 4249.1637 cycles]
                        thrpt:  [6.6393 cpb 6.6278 cpb 6.6160 cpb]
rc4ok/512B key/128B out (random)                                                                             
                        time:   [4280.2904 cycles 4289.0777 cycles 4297.3391 cycles]
                        thrpt:  [6.7146 cpb 6.7017 cpb 6.6880 cpb]

rc4ok/512B key/512B out (cached)                                                                             
                        time:   [5872.9725 cycles 5882.1885 cycles 5892.3259 cycles]
                        thrpt:  [5.7542 cpb 5.7443 cpb 5.7353 cpb]
rc4ok/512B key/512B out (random)                                                                             
                        time:   [5928.2071 cycles 5941.1224 cycles 5953.0667 cycles]
                        thrpt:  [5.8135 cpb 5.8019 cpb 5.7893 cpb]

rc4ok/512B key/2048B out (cached)                                                                              
                        time:   [12446.3101 cycles 12466.3025 cycles 12484.1382 cycles]
                        thrpt:  [4.8766 cpb 4.8696 cpb 4.8618 cpb]
rc4ok/512B key/2048B out (random)                                                                             
                        time:   [12533.1298 cycles 12549.1192 cycles 12565.9494 cycles]
                        thrpt:  [4.9086 cpb 4.9020 cpb 4.8958 cpb]

rc4ok/512B key/8192B out (cached)                                                                             
                        time:   [38646.2054 cycles 38674.0255 cycles 38702.6669 cycles]
                        thrpt:  [4.4465 cpb 4.4432 cpb 4.4401 cpb]
rc4ok/512B key/8192B out (random)                                                                             
                        time:   [38780.2013 cycles 38808.2085 cycles 38839.4959 cycles]
                        thrpt:  [4.4623 cpb 4.4587 cpb 4.4554 cpb]
```

## Usage

Using RC4OK PRNG is fairly easy.

1) Add `rc4ok` to the *[dependencies]* section of the Cargo.toml file of your project.

```toml
[dependencies]
rc4ok = { git = "https://github.com/itzmeanjan/rc4ok" }
```

2) Initialize RC4OK pseudo-random number generator with a non-empty key i.e. seed.

```rust
use rc4ok;

fn main() {
    const SEED_LEN: usize = 16;
    const OUT_LEN: usize = 32;

    let seed = vec![0xffu8; SEED_LEN];
    let mut out = vec![0u8; OUT_LEN];

    // Seed PRNG
    let mut rc4ok_prng = rc4ok::RC4ok::init(&seed);
    // ...
}
```

3) Request arbitrary many pseudo-random bytes from PRNG object.

```rust
// Generate pseudo-random bytes
rc4ok_prng.generate(&mut out);
```

4) You can add some entropy into the RC4OK PRNG state from time to time.

> **Warning**
RC4OK state is not *yet* thread-safe so you can't spawn a thread to harvest entropy and add that to the state of RC4OK PRNG from time to time.

```rust
let mut entropy = 0u16;          // harvest 16 -bit entropy
rc4ok_prng.add_entropy(entropy); // Add entropy
```

5) Finally you can reset the state of an existing RC4OK PRNG and reinit it with a new non-empty seed.

```rust
let another_seed = vec![0x0fu8; SEED_LEN + 1]; // Populate another seed
rc4ok_prng.reset(&another_seed);               // Re-seed PRNG
```

I'm maintaining a program (see [src/main.rs](./src/main.rs)) which can be invoked as a binary and requested for producing arbitrary many psuedo-random bytes given a non-empty seed string.

> **Note**
`rc4ok` binary executable writes requested-many or arbitrary-many pseudo-random bytes directly onto **STDOUT** device, hence you may want to pipe ( read more @ https://en.wikipedia.org/wiki/Pipeline_(Unix) ) the output to a file or another program.

```bash
cargo run --release -- -h                             # For showing help text
cargo run --release -- -b 256 "this is a seed phrase" # For requesting n (=256) pseudo-random bytes, given a seed string
cargo run --release -- -s "this is a seed phrase"     # For requesting arbitrary pseudo-random bytes, given a seed string
```
