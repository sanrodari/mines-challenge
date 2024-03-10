# Mines challenge

This repo contains the solution for this challenge: https://web.archive.org/web/20231211093416/https://gdlauncher.com/en/careers/rust/

## Naive implementation

I started with a naive implementation that helped me to establish a performance and correctness baseline.

It does a quadratic search in a buffer of the latest (`MINE_LENGTH`) numbers.

We can think of this algorithm to have `O(n * m^2)` time complexity (ignoring any vector regrows that might occur).
`n` being the amount of numbers to process, and `m` the `MINE_LENGTH`.

## Optimized implementation

Using a sorted map of numbers that represent the latest (`MINE_LENGTH`) numbers,
I can now search for the remainder efficiently without the need of a quadratic search.

The time complexity for this algorithm is `O(n * m log m)`. I added further optimization based on the fact the map is sorted:

* When looking for the remainder ends up with a number greater than its half, it halts the search.
* The max and min sum from the sorted map can be calculated in constant time, allowing to halt the search if the candidate is not within this range.

In practice, and with the sample input provided, the reminder was found almost in the first iteration,
making it close to `O(n * log m)` time complexity.

## Some quick and dirty benchmarking

Using this I timed and compared the results of both algorithms:

```sh
$ ls -lh big.txt
-rw-r--r--  1 sanrodari  staff   7.7M Mar  9 12:54 big.txt
$ cargo build --release
$ time ./mines naive big.txt > 1.txt \
  && time ./mines optimized big.txt > 2.txt \
  && git diff --no-index 1.txt 2.txt

target/release/mines naive big.txt > 1.txt      0.34s user 0.02s system 98% cpu 0.363 total
target/release/mines naive big.txt > 1.txt      0.34s user 0.02s system 98% cpu 0.361 total
target/release/mines naive big.txt > 1.txt      0.35s user 0.02s system 92% cpu 0.397 total
target/release/mines naive big.txt > 1.txt      0.34s user 0.02s system 98% cpu 0.361 total
target/release/mines naive big.txt > 1.txt      0.34s user 0.02s system 99% cpu 0.359 total
target/release/mines naive big.txt > 1.txt      0.34s user 0.02s system 99% cpu 0.360 total
target/release/mines naive big.txt > 1.txt      0.36s user 0.02s system 98% cpu 0.387 total
target/release/mines naive big.txt > 1.txt      0.35s user 0.02s system 98% cpu 0.372 total

target/release/mines optimized big.txt > 2.txt  0.13s user 0.02s system 97% cpu 0.153 total
target/release/mines optimized big.txt > 2.txt  0.13s user 0.02s system 97% cpu 0.156 total
target/release/mines optimized big.txt > 2.txt  0.13s user 0.02s system 98% cpu 0.153 total
target/release/mines optimized big.txt > 2.txt  0.13s user 0.02s system 98% cpu 0.157 total
target/release/mines optimized big.txt > 2.txt  0.14s user 0.02s system 97% cpu 0.158 total
target/release/mines optimized big.txt > 2.txt  0.13s user 0.02s system 97% cpu 0.156 total
target/release/mines optimized big.txt > 2.txt  0.13s user 0.02s system 98% cpu 0.152 total
target/release/mines optimized big.txt > 2.txt  0.13s user 0.02s system 98% cpu 0.149 total
```

We can see a little more than 50% performance gains from the `naive` to the `optimized` version.

You can see the results of the mines that will crumble [here](./results.txt).
