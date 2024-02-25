# Compilation
This library is expected to work with two L\* (and its derivative) implementation: [pylstar](https://github.com/gbossert/pylstar) in python and [Learnlib](https://github.com/LearnLib/learnlib) in Java.
## Compilation to python library
```
Cargo build --release 
```
Then you can go take the file libmapper.so in path-to-project/target/release/libmapper.so

## Compilation to java library
```
cargo build  --release --features "java" --no-default-features
```
Then you can go take the file libmapper.so in path-to-project/target/release/libmapper.so
