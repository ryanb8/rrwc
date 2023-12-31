#!/usr/bin/env bash

die () {
    echo >&2 "$@"
    exit 1
}

[ "$#" -eq 2 ] || die "2 argument required, $# provided"

filenames=("hello_world.txt" "moby_dick_chapter_1_only.txt" "moby_dick_chapter_1_10.txt" "moby_dick_chapter_1_25.txt" "moby_dick_chapter_1_50.txt" "moby_dick.txt" "kjv_1x.txt" "kjv_10x.txt" "kjv_100x.txt")

gen_call_and_name () {
    if [[ $1 == "wc" ]]; then
        gen_wc_call $2
    elif [[ $1 == "uuwc" ]]; then
        gen_uu_wc_call $2
    else
        gen_rrwc_call $2 $1
    fi
}

gen_rrwc_call () {
    echo "'""./target/release/rrwc benchmark_texts/$1 $2""'"
}

gen_wc_call () {
    echo "'""wc benchmark_texts/$1""'"
}

gen_uu_wc_call () {
    echo "'""${HOME}/.cargo/bin/coreutils wc benchmark_texts/$1""'"
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
    this_cmd="hyperfine --warmup 10 -N --export-json $benchmark_folder/$benchmark_fn $call_1 $call_2"
    echo "running $this_cmd"
    eval "$this_cmd"
    echo ""

done
