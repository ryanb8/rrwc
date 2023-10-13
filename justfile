help:
  # Just commands:
  just --list

download-test-texts:
  ./script/download_text_files.sh

benchmark: compile
  ./script/benchmark.sh

compile:
  cargo build --release
