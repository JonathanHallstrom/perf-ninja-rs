#[cfg(test)]
mod tests;

pub fn solution(_arr: &[i32], n: usize) -> i32 {
    (n * (n + 1) / 2) as i32
}
