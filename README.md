# X-OR SHIFT RNG

using a **seed: `Vec<u32>`** of length 4, any number of random integers can be generated.

- `crate::randi(n: u32, seed: &mut Vec<u32>) -> Vec<u32>`
- `crate::randi_max(n: u32, max: u32, seed: &mut Vec<u32>) -> Vec<u32>`