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

profile-small:
  @echo 'Profiling with version $0'
  cargo instruments -t time --bin ryan_wc --profile release-with-debug benchmark_texts/moby_dick.txt $0
