# mandelblot


Programming Rust
Code examples for the book Programming Rust, from O'Reilly
https://github.com/ProgrammingRust/mandelbrot

## new

```sh
> cargo new --bin mandelbrot
```

## run
```sh
> cd path/to/mandelbrot
> cargo run mandelbrot.png 1000x750 -1.20,0.35 -1,0.20
```

### args

`<outputfilename> <widthxheight> <upper_left> <lower_right>`

## test
```sh
> cargo test
```

## build

```sh
> cargo build --release
> .\target\release\mandelbrot  mandelbrot.png 1000x750 -1.20,0.35 -1,0.20
```