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
* ar (binutils)
* Python 3


## Quick start

```console
$ git clone git://github.com/bhuztez/porus.git
$ cd porus
$ pip3 install -r requirements.txt
$ python3 -mix submit -w solutions/HR/solve-me-first.rs
[SUBMIT] solutions/HR/solve-me-first.rs
[COMPILE] target/x86_64-unknown-linux-gnu/release/libporus.rlib
[COMPILE] src/bin/linkbc.rs
[COMPILE] solutions/HR/solve-me-first.rs
[COMPILE] solutions/HR/solve-me-first.bc
HR (hackerrank.com)
Username: your_username
Password:
[SUBMIT] solutions/HR/solve-me-first.rs: Accepted
$
```
