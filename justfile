set positional-arguments

help:
  # Just commands:
  just --list

download-test-texts:
  ./script/download_text_files.sh

benchmark version_1 version_2:
  @echo 'Use form `just benchmark <wc_or_version> <wc_or_version>`'
  @echo recieved version_1 = $1 and version_2 = $2
  just compile
  ./script/benchmark.sh $1 $2

compile:
  cargo build --release

profile-compile:
  cargo build --profile=release-with-debug

profile-small version:
  @echo "Profiling with version $1"
  cargo instruments -t time --bin rrwc --profile release-with-debug benchmark_texts/moby_dick.txt $1

profile-large version:
  @echo "Profiling with version $1"
  cargo instruments -t time --bin rrwc --profile release-with-debug benchmark_texts/kjv_10x.txt $1
