## Wordcount

My attempt to write a wc in rust. Focusing on performance/speed book style text files (not xml, code, html, etc).

Using hyperfine to benchmark vs `wc` and `wc` for judging correctness. (Note: that I think `wc` has a bug with greek letters, the file with `εξω βελους` returns 3 words, but it should only be 2 I think?!?)

Currently only a naive rust implementation exists.

## Requirenments

- Rustc 1.63+ (I think), built and tested on rustc 1.73.0
- [Just](https://github.com/casey/just), if you want to run the just commands. [Install instructions by platform]
(https://github.com/casey/just#installation).

## Running benchmarks

Just run `just benchmark`, which runs the benchmark script in 

### Profiling

Only supports Mac. Requires installation of [Xcode](https://apps.apple.com/us/app/xcode/id497799835?mt=12). Requires [cargo-instruments](https://github.com/cmyr/cargo-instruments). Install with `brew install cargo-instruments`.


# TODO

- write up benchmark stuff
- Start performance tuning in rust (starting with Rayon parallel)
- Write tests to compare WC and rust output



