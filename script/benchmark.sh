#!/usr/bin/env bash

mkdir benchmark_output
benchmark_run=$(date +%Y%m%d_%H%M%S)
benchmark_folder=benchmark_output/"$benchmark_run"
mkdir "$benchmark_folder"
hyperfine --warmup 100  -N --export-json "$benchmark_folder"/moby_dick.json 'wc example_txt_files/moby_dick.txt'
