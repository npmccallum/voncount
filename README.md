[![Build Status](https://travis-ci.org/npmccallum/voncount.svg?branch=master)](https://travis-ci.org/npmccallum/voncount)
![Rust Version](https://img.shields.io/badge/rustc-v1.13%2B-blue.svg)

Welcome to `voncount` - a Rust crate for counters.

Like the lovable Count von Count from Sesame Street, the `voncount` crate loves
to count things. We provide the `Counter` trait which can be implemented on
types which try to count things. We also provide two structs which implement
the `Counter` trait:
  * `ReadCounter`
  * `WriteCounter`
