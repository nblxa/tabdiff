# tabdiff
Command line tool showing the row-by-row and field-by-field difference between two CSV files.

## Build
Download the Rust build toolchain and build. 
```sh
$ curl https://sh.rustup.rs -sSf | sh
$ cargo build --release
```

## Run
🌈 The console output is colorized.
```sh
$ target/release/tabdiff -l key -r key example/a.csv example/b.csv
+-------+----------------------------+----------------------------+
| Diff  | code/tabdiff/example/a.csv | code/tabdiff/example/b.csv |
+=======+============================+============================+
| left  | key   : all my base        |                            |
|       | value : belong to you      |                            |
+-------+----------------------------+----------------------------+
| right |                            | key   : all your base      |
|       |                            | value : belong to me       |
+-------+----------------------------+----------------------------+
| eq    | key   : roses              | key   : roses              |
|       | value : red                | value : red                |
+-------+----------------------------+----------------------------+
| diff  | key   : violets            | key   : violets            |
|       | value : blue               | value : green              |
+-------+----------------------------+----------------------------+
```
