# Rust example for panic handling

This Rust example is for panics handling, including `divided by zero`, `InvalidDigit` and `out of index range panic`. There are 3 times inputs from IO, the user can input both `numerator` and `denominator`. Rust code do the division calculate by `numerator/denominator`, then store the result into an `i32` array with fixed lenght `3` and iterate this array at last.

Note: I make an implicit panic when iterating the array by indexing the array from 0 to 3, and this could make the `out of index range panic` occur.

## Directories Description

- `rust-panic-without-handling` is the example whithout handling any panic
- `rust-panic-with-handling` is the example to handle all panics, including `divided by zero`, `InvalidDigit` and `out of index range panic`.


## Example of `rust-panic-without-handling` 

### Build the binary in folder `rust-panic-without-handling`
```
cd rust-panic-without-handling
cargo build
```
### `divided by zero` panic
```
$ target/debug/rust-panic-without-handling 

 ###### Divide by zero ###### 

Please input the numerator: 435
Please input the denominator: 65
Result is: 6
##########################################
Please input the numerator: 45
Please input the denominator: 0
thread 'main' panicked at src/main.rs:26:35:
attempt to divide by zero
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

### `InvalidDigit` panic
```
$ target/debug/rust-panic-without-handling 

 ###### Divide by zero ###### 

Please input the numerator: 675
Please input the denominator: 45
Result is: 15
##########################################
Please input the numerator: 33a
thread 'main' panicked at src/main.rs:16:55:
Invalid input: ParseIntError { kind: InvalidDigit }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

### `out of index range panic`
```
$ target/debug/rust-panic-without-handling 

 ###### Divide by zero ###### 

Please input the numerator: 878
Please input the denominator: 3
Result is: 292
##########################################
Please input the numerator: 653
Please input the denominator: 53
Result is: 12
##########################################
Please input the numerator: 967
Please input the denominator: 54
Result is: 17
##########################################

 @@@@@@ Iteration @@@@@@ 

Iterate Element: 292
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
Iterate Element: 12
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
Iterate Element: 17
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
thread 'main' panicked at src/main.rs:34:41:
index out of bounds: the len is 3 but the index is 3
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

**Note: Showing as above, the whole process would exit once any panic occurs. And it's impossible for us to see the output of `Complete the panic handle examples!`**

## Example of `rust-panic-with-handling` 

### Build the binary in folder `rust-panic-with-handling`
```
cd rust-panic-with-handling
cargo build
```
### `divided by zero` and `out of index range panic` panics
```
$ target/debug/rust-panic-with-handling 

 ###### Divide by zero ###### 

This is the 0th to handle panic.
Please input the numerator: 432
Please input the denominator: 43
No panic occur and the result is: 10
##########################################
This is the 1th to handle panic.
Please input the numerator: 5343
Please input the denominator: 0
thread 'main' panicked at src/main.rs:30:13:
attempt to divide by zero
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
Caught panic: attempt to divide by zero
##########################################
This is the 2th to handle panic.
Please input the numerator: 632
Please input the denominator: 34
No panic occur and the result is: 18
##########################################

 @@@@@@ Iteration @@@@@@ 

Iterate Element: 10
No panic occur for this iteration
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
Iterate Element: 0
No panic occur for this iteration
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
Iterate Element: 18
No panic occur for this iteration
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
thread 'main' panicked at src/main.rs:56:45:
index out of bounds: the len is 3 but the index is 3
Caught panic of unknown type
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
Complete the panic handle examples!
```

### `InvalidDigit` and `out of index range panic` panics
```
$ target/debug/rust-panic-with-handling 

 ###### Divide by zero ###### 

This is the 0th to handle panic.
Please input the numerator: 5433
Please input the denominator: 43
No panic occur and the result is: 126
##########################################
This is the 1th to handle panic.
Please input the numerator: 532
Please input the denominator: 34e
thread 'main' panicked at src/main.rs:27:63:
Invalid input: ParseIntError { kind: InvalidDigit }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
Caught panic of unknown type
##########################################
This is the 2th to handle panic.
Please input the numerator: 5432
Please input the denominator: 35
No panic occur and the result is: 155
##########################################

 @@@@@@ Iteration @@@@@@ 

Iterate Element: 126
No panic occur for this iteration
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
Iterate Element: 0
No panic occur for this iteration
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
Iterate Element: 155
No panic occur for this iteration
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
thread 'main' panicked at src/main.rs:56:45:
index out of bounds: the len is 3 but the index is 3
Caught panic of unknown type
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
Complete the panic handle examples!
```

**Note: Showing as above, the whole process would not exit even any panic occurs. And we always can see the output of `Complete the panic handle examples!`**
