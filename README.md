# X-OR SHIFT RNG

using a **seed: `Vec<u32>`** of length 4, any number of random integers can be generated.

- `crate::randi(n: u32, seed: &mut Vec<u32>) -> Vec<u32>`
- `crate::randi_max(n: u32, max: u32, seed: &mut Vec<u32>) -> Vec<u32>`

you must save the seed, and use it on every call of the functions. it is changed during the function execution, and that is what makes it random. you can reuse the same seed for both functions.

if you use the same numbers on every call of the function (i.e. `vec!(1, 2, 3, 4)` as a seed on every call) instead of reusing the seed, you'll get the same "random" numbers, in the same order.
