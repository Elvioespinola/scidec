/*
 * MIT License
 *
 * Copyright (c) 2022 Dariusz Depta
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

//! Benchmarks.

#![feature(test)]

extern crate test;

use scidec::number_from_string;
use test::Bencher;

#[bench]
fn bench_number_from_string_0001(b: &mut Bencher) {
  b.iter(|| {
    let _ = number_from_string("12");
  });
}

#[bench]
fn bench_number_from_string_0002(b: &mut Bencher) {
  b.iter(|| {
    let _ = number_from_string("938475E-03");
  });
}

#[bench]
fn bench_number_from_string_0003(b: &mut Bencher) {
  b.iter(|| {
    let _ = number_from_string("0.00003E-02");
  });
}

#[bench]
fn bench_number_from_string_0004(b: &mut Bencher) {
  b.iter(|| {
    let _ = number_from_string("9999999999999999999999999999999999");
  });
}
