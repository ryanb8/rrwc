help:
  # Just commands:
  just summary

benchmark: compile
  ./script/benchmark.sh

compile:
  cargo build --release
