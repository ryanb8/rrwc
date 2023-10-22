#!/usr/bin/env bash

mkdir benchmark_texts
MYTMPDIR="$(mktemp -d)"
trap 'rm -rf -- "$MYTMPDIR"' EXIT

echo "Creating Hello World"
echo "Hello World" > benchmark_texts/hello_world.txt

echo "Downloading Moby Dick"
echo "Running \`curl -iH "Accept: text/plain" -o benchmark_texts/moby_dick.txt https://www.gutenberg.org/files/2701/old/moby10b.txt\`"
curl -iH "Accept: text/plain" -o benchmark_texts/moby_dick.txt https://www.gutenberg.org/files/2701/old/moby10b.txt

echo "Creating Moby Dick 1st chapter only"
cp benchmark_texts/moby_dick.txt benchmark_texts/moby_dick_chapter_1_only.txt
perl -ne '/CHAPTER 1\s/../CHAPTER 2\s/ and print' benchmark_texts/moby_dick.txt >  benchmark_texts/moby_dick_chapter_1_only.txt

kjvdir=$MYTMPDIR/kjv
mkdir "$kjvdir"
echo "Downloading KJV Bible"
echo "Running: \`curl -Lo "$kjvdir"/tmp_kjv.zip \"https://archive.org/compress/kjv-text-files/formats=TEXT&file=/kjv-text-files.zip\"\`"
curl -Lo $kjvdir/tmp_kjv.zip "https://archive.org/compress/kjv-text-files/formats=TEXT&file=/kjv-text-files.zip"
echo "Running unzip $kjvdir/tmp_kjv.zip -d $kjvdir"
unzip -q "$kjvdir"/tmp_kjv.zip -d "$kjvdir"

echo "Generating KJV 1x"
rm -f benchmark_texts/kjv_1x.txt
echo "$kjvdir"
cat "$kjvdir"/*.txt > benchmark_texts/kjv_1x.txt

echo "Generating KJV 10x"
rm -f benchmark_texts/kjv_10x.txt
for i in {1..10}
do
   cat benchmark_texts/kjv_1x.txt >> benchmark_texts/kjv_10x.txt
done

echo "Generating KJV 100x"
rm -f benchmark_texts/kjv_100x.txt
for i in {1..10}
do
   cat benchmark_texts/kjv_10x.txt >> benchmark_texts/kjv_100x.txt
done

