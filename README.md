## Rust of life

This is a simple game of life implemented in Rust using Piston and OpenGL.

This is made for educational purposes, it's probably not the best rust
code in the world, but we all had to begin somewhere. ;)


### Running

Pass the path to an input file as the first positional argument:

```bash
./rust_of_life examples/input.txt # optionally cargo run examples/input.txt
```

### Input file

An example input file is included into the [examples](examples) directory.

The format of the input file mimics the grid of the game of life. Each
character represents a single cell.

Make sure every line has the same amount of characters.

You can use any desired character to represent dead cells (and they do not need
to be the same). The living cells are represented with the letter `o`
(lowercase).

See the [examples/input.txt](examples/input.txt).

## License

This is licensed under [MIT](LICENSE).
