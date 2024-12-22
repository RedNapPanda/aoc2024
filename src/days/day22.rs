const MODULO: i64 = 16777216;

pub fn solve1(lines: &[String]) -> i64 {
    let mut sum = 0;
    for line in lines {
        let mut secret = line.parse::<i64>().unwrap();
        for _ in 0..2000 {
            secret ^= secret << 6;
            secret %= MODULO;
            secret ^= secret >> 5;
            secret %= MODULO;
            secret ^= secret << 11;
            secret %= MODULO;
        }
        sum += secret;
    }
    sum
}

pub fn solve2(lines: &[String]) -> i64 {
    0
}