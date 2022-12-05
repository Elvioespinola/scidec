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

//!

use crate::{number_from_string, Number};

mod number_from_string;

fn num_fin(input: &str, sign: bool, w1: u64, w0: u64, exp: i32) {
  match number_from_string(input) {
    Number::Fin(actual_sign, actual_w1, actual_w0, actual_exponent) => {
      assert_eq!(sign, actual_sign);
      assert_eq!(w1, actual_w1);
      assert_eq!(w0, actual_w0);
      assert_eq!(exp, actual_exponent);
    }
    other => panic!("expected number, actual value is {:?}", other),
  }
}

fn num_inf(input: &str, sign: bool) {
  match number_from_string(input) {
    Number::Inf(actual_sign) => assert_eq!(sign, actual_sign),
    other => panic!("expected infinity, actual value is {:?}", other),
  }
}

fn num_nan(input: &str, signaling: bool) {
  match number_from_string(input) {
    Number::NaN(actual_signaling) => assert_eq!(signaling, actual_signaling),
    other => panic!("expected NaN, actual value is {:?}", other),
  }
}
