# Boss Baby's Revenge (Problem 1)

This package contains the function `solve` which determines if the Boss Baby is a Good Boy or not.

## Running

The easiest way to run this solution is using cargo. You will be prompted with input in console.
```sh
cargo run
input: SSR
result: Bad boy
```
### Constrains
- Input string must contains only `S` and `R`
- The length is in [1, 1000000] range.

## Solution

The code divides input into groups of leading `S`(shot) and trailing `R`(Retaliation). 
```
SSRRSRSRRR = SSRR/SR/SRRR
```
In a group, we add the number of `S` into `count` variable. We also subtract with the number of `R`.

```rust
if ch == b'S' {
    count = count + 1; // add to retaliation count
} else {
    count = count - 1; // minus from retaliate shot
}
```

After reading 1 group, we check if all the shot have been retaliated. If not (`count` > 0), we return `Bad boy`.

```rust
if ch == b'S' {
    if last_elem_is_r {
        if count > 0 {
            return "Bad boy";
        }
        count = 0;
    }
}
```

Then we reset the `count` and move to the next group until finish.

## Time complexity

We breaks input into an iterator over `Bytes` and goes through it `once`:

```rust
let mut it = input.bytes();

for ch in it {
    ...
}
```

Hence, the worst case time complexity is: `O(n)`

## Memory complexity

The code uses an `iterator` and some primitive data type in addition to the input array itself:

```rust
let mut it = input.bytes();
    
let mut last_elem_is_r = false;
let mut count = 1;
```

Hence, the memory complexity (disregard of the input) is: `O(1)`
