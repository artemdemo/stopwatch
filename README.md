# Time (Stopwatch)

This is a fun project made to test 2D rendering with [notan](https://crates.io/crates/notan).

Everything is tested on Mac OS and `rustc` 1.83.0 (90b35a623 2024-11-26).

In order to build this project. You need to have following packages installed on your system:

* `cmake`

**Keyboard shortcuts**

* `S` - switch to stopwatch, start/pause the stopwatch
* `R` - reset the stopwatch
* `T` - go back to showing time

In order to count down, while in stopwatch, just start entering number to count down from.
Then press `S`.

**WASM support**

Everything is tasted with [`trunk`](https://trunkrs.dev/).
You need to install it (I used `brew install trunk`), then you can build and serve your code.
Just run following:

```
trunk serve
```

----

Backlog:

* Display current state (Time or Stopwatch).
* Display whether stopwatch is running and in what direction.
* After that you actually can switch back to time without pausing stopwatch.
* Figure out how to build WASM without `trunk`

