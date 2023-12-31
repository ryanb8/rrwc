
- [Ryan's Rust Wordcount (rrwc)](#ryans-rust-wordcount-rrwc)
  - [Installation](#installation)
  - [Performance](#performance)
  - [Requirenments](#requirenments)
  - [Running benchmarks](#running-benchmarks)
    - [Profiling](#profiling)
- [TODO (which likely won't happen - this was for fun)](#todo-which-likely-wont-happen---this-was-for-fun)

# Ryan's Rust Wordcount (rrwc)

My attempt to write a simple word counter in rust. Focusing on performance/speed book style text files (not xml, code, html, etc). Currently we are ~1.8-2.1x more performant than `wc` on very large files, while wc is faster on very small files. Speed parity is at files around 200kb in size.

To be fair, wc is probably more portable and supports a few options that my version doesn't yet.

We are using hyperfine to benchmark vs `wc` for benchmarking and `wc` for judging correctness. (Note: that I think `wc` has a bug with greek letters, a file with `εξω βελους` returns 3 words with wc, but it should only be 2 I think?!?)

There are a few rust implementations, though currently `low_level_buf_reader` is our best performing variant.

## Installation

You should be able to `just compile` and have it built. You can install it if you'd like.

## Performance

Benchmarks run on 2019 MBP 2.6 GHz 6-Core Intel Core i7 comparing `wc` and my `low_level_buf_reader` version.

| File                    | File Size | wc Time | ryans_rust_wc Time | Winner   | rrwc perf improvement |
| ----------------------- | --------- | ------- | ------------------ | -------- | --------------------- |
| "Hello world"           | 12 b      | 2.0ms   | 2.7ms              | wc       | 0.75                  |
| Moby Dick's 1st Chapter | 13 Kb     | 2.2ms   | 2.7ms              | wc       | 0.82                  |
| Moby Dick Chapters 1-10 | 103 Kb    | 2.7ms   | 2.9ms              | wc       | 0.93                  |
| Moby Dick Chapters 1-25 | 235 Kb    | 3.4ms   | 3.2ms              | **rrwc** | **1.06**              |
| Moby Dick Chapters 1-50 | 489 Kb    | 4.8ms   | 3.9ms              | **rrwc** | **1.23**              |
| Moby Dick               | 1.3 Mb    | 9.1ms   | 5.8ms              | **rrwc** | **1.58**              |
| KJV Bible               | 4.3 Mb    | 25.0ms  | 13.4ms             | **rrwc** | **1.87**              |
| 10x KJV Bible           | 43.3 Mb   | 223.4ms | 107.4ms            | **rrwc** | **2.08**              |
| 100x KJV Bible          | 432.8 Mb  | 2.249s  | 1.083s             | **rrwc** | **2.08**              |

## Requirenments

- Rustc 1.63+ (I think), built and tested on rustc 1.73.0
- [Just](https://github.com/casey/just), if you want to run the just commands. [Install instructions by platform](https://github.com/casey/just#installation).

## Running benchmarks

Run `just download-test-texts` to download and generate all the texts for benchmarking (5-6Mbg of downloads, close to 500mb once unzipped+generated).

Run `just benchmark <version_1> <version_2>` to compare two versions with the benchmark script. Currently supported versions are:
- `wc` - the GNU tool
- My implementations:
  - `naive` - the 2 minute version
  - `low_level_buf_reader` - a low level loop that processes character by character using a bufreader for io. This is the best implementation so far.
- My misfit toys- not trustworthy implementations: 
  - `low_level_full_file`
  - `low_level_custom_buffer`
  - `naive_rayon`
  - `naive_rayon_big_buf`
  - `naive_full_file`
  - `full_file_via_buf`

### Profiling

Under progress. Use at your own risk (like the whole project, but especially this).

Only supports Mac. Requires installation of [Xcode](https://apps.apple.com/us/app/xcode/id497799835?mt=12). Requires [cargo-instruments](https://github.com/cmyr/cargo-instruments). Install with `brew install cargo-instruments`.

Run `just profile-small <version>` or `just profile-large <version>`. `wc` is not supported for profiling.


# TODO (which likely won't happen - this was for fun)

- Performance tuning on small files
- unit tests
- Write tests to compare WC and rust output
- pretty graphs
- Make profiling useful and figure it out more
- Test some more obscure-y edge cases - emojis, etc
