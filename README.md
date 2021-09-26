# tabdiff
Command line tool showing the row-by-row- field-by-field difference between two CSV files.

## Build
```sh
curl https://sh.rustup.rs -sSf | sh
cargo build --release
```

## Run
```sh
code/tabdiff/target/release/tabdiff  code/tabdiff/example/a.csv code/tabdiff/example/b.csv -l key -r key 
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
