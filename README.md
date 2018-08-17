# porus

[![Build Status](https://travis-ci.org/bhuztez/porus.svg?branch=master)](https://travis-ci.org/bhuztez/porus)
[![Coverage Status](https://coveralls.io/repos/github/bhuztez/porus/badge.svg?branch=master)](https://coveralls.io/github/bhuztez/porus?branch=master)

porus is Rust library designed for competitive programming, especially
for being used by solutions submitted to online judges. So that you
don't have to copy and paste library code into your solution.


## Requirements

* Rust nightly
  * i686-pc-windows-gnu
  * i686-unknown-linux-gnu
  * x86_64-pc-windows-gnu
  * x86_64-unknown-linux-gnu
* Python 3.5+


## Quick start

```console
$ git clone git://github.com/bhuztez/porus.git
$ cd porus
$ pip3 install --user -r requirements.txt
$ python3 -mix submit -w solutions/AOJ/ITP1/ITP1_1_A.rs
[SUBMIT] solutions/AOJ/ITP1/ITP1_1_A.rs
[COMPILE] target/x86_64-unknown-linux-gnu/release/libporus.rlib
AOJ (judge.u-aizu.ac.jp)
User ID: your_username
Password:
[SUBMIT] solutions/AOJ/ITP1/ITP1_1_A.rs: Accepted (Memory: 2000, Time: 0, Length: 1380)
$
```

## Examples

* [AOJ](AOJ.md) ([AIZU ONLINE JUDGE](http://judge.u-aizu.ac.jp/onlinejudge/))
* [LC](LC.md) ([LeetCode](https://leetcode.com/))