## Wordcount

My attempt to write a wc in rust. Focusing on book style text files (not xml, code, html, etc).

Using hyperfine to benchmark vs WC.

Currently only a naive rust implementation exists.


## Requirenments

- Rustc 1.63+ (I think), built and tested on rustc 1.73.0
- [Just](https://github.com/casey/just), if you want to run the just commands. [Install instructions by platform]
(https://github.com/casey/just#installation).

### Profiling

Only supports Mac. Requires installation of [Xcode](https://apps.apple.com/us/app/xcode/id497799835?mt=12). Requires [cargo-instruments](https://github.com/cmyr/cargo-instruments). Install with `brew install cargo-instruments`.


# TODO

- Script download of moby dick 
- write up benchmark stuff
- Start performance tuning in rust (starting with Rayon parallel)
- Write tests to compare WC and rust output

Benchmark will download KJV bible 


