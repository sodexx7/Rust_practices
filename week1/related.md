https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#variable-scope

In addition, there’s a design choice that’s implied by this: Rust will never automatically create “deep” copies of your data. Therefore, any automatic copying can be assumed to be inexpensive in terms of runtime performance.

borrowing
It’s quite annoying that anything we pass in also needs to be passed back if we want to use it again,

Other reference
https://adventofcode.com/
https://nnethercote.github.io/perf-book/title-page.html
