set positional-arguments

help:
  # Just commands:
  just --list

download-test-texts:
  ./script/download_text_files.sh

benchmark: compile
  ./script/benchmark.sh

compile:
  cargo build --release

profile-compile:
  cargo build --profile=release-with-debug

profile-small version:
  @echo "Profiling with version $1"
  cargo instruments -t time --bin ryan_wc --profile release-with-debug benchmark_texts/moby_dick.txt $1

profile-large version:
  @echo "Profiling with version $1"
  cargo instruments -t time --bin ryan_wc --profile release-with-debug benchmark_texts/kjv_100x.txt $1
