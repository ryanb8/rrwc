#!/usr/bin/env bash

# TODO: Script to accept two different versions (WC or any of the versions in the rust binary)

die () {
    echo >&2 "$@"
    exit 1
}

[ "$#" -eq 2 ] || die "2 argument required, $# provided"


filenames=("moby_dick.txt" "kjv_1x.txt" "kjv_10x.txt" "kjv_100x.txt")


gen_call_and_name () {
    if [[ $1 == "wc" ]]; then
        gen_wc_call $2
    else
        gen_ryan_wc_call $2 $1
    fi
}

gen_ryan_wc_call () {
    echo "'""./target/release/ryan_wc benchmark_texts/$1 $2""'"
}

gen_wc_call () {
    echo "'""wc benchmark_texts/$1""'"
}


mkdir benchmark_output
benchmark_run=$(date +%Y%m%d_%H%M%S)
benchmark_folder=benchmark_output/"$benchmark_run"__$1__$2
mkdir "$benchmark_folder"
echo ""

for filename in "${filenames[@]}"
do
    echo "$filename"
    benchmark_fn=${filename//.txt/.json}
    call_1=$(gen_call_and_name "$1" "$filename")
    call_2=$(gen_call_and_name "$2" "$filename")
    this_cmd="hyperfine --warmup 10 --export-json $benchmark_folder/$benchmark_fn $call_1 $call_2"
    echo "running $this_cmd"
    eval "$this_cmd"
    echo ""

done
