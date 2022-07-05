// n: how many numbers to generate; max: max value of generated numbers; of seed: Vec<u32> of length 4.
pub fn randi_max(n: u32, max: u32, seed: &mut Vec<u32>) -> Vec<u32> {
    let mut result: Vec<u32> = Vec::new();

    for _ in 0..n {
        let tmp: u32 = seed[0] ^ (seed[0] << 15);
        seed[0] = seed[1];
        seed[1] = seed[2];
        seed[2] = seed[3];
        seed[3] = ((seed[3] ^ (seed[3] >> 21)) ^ (tmp ^ (tmp >> 4))) % max;

        result.push(seed[3]);
    }

    result
}

// n: how many numbers to generate; seed: Vec<u32> of length 4.
pub fn randi(n: u32, seed: &mut Vec<u32>) -> Vec<u32> {
    let mut result: Vec<u32> = Vec::new();

    for _ in 0..n {
        let tmp: u32 = seed[0] ^ (seed[0] << 15);
        seed[0] = seed[1];
        seed[1] = seed[2];
        seed[2] = seed[3];
        seed[3] = (seed[3] ^ (seed[3] >> 21)) ^ (tmp ^ (tmp >> 4));

        result.push(seed[3]);
    }

    result
}
