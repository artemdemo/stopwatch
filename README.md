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

----

ToDo:

* Render it in the browser (WASM).
  * https://github.com/Nazariglez/notan?tab=readme-ov-file#webassembly
* Dark / White theme
  * https://crates.io/crates/dark-light

----

Backlog:

* Display current state (Time or Stopwatch).
* Display whether stopwatch is running and in what direction.
* After that you actually can switch back to time without pausing stopwatch.
