#![feature(test)]

use rand::prelude::*;

pub fn monte_carlo_pi(iterations: usize) -> f32 {
    let mut inside_circle = 0;
    for _ in 0..iterations {
        let x: f32 = random::<f32>();
        let y: f32 = random::<f32>();
        if x.powi(2) + y.powi(2) <= 1_f32 { inside_circle += 1; }
    }
    (4_f32 * inside_circle as f32) / iterations as f32
}

pub fn monte_carlo_pi_rec(iterations: usize) -> f32 {
    fn go(iterations: usize, counter: usize, to: usize) -> f32 {
        match iterations {
            0 => (4_f32 * counter as f32) / (to as f32),
            _ => {
                let x: f32 = random::<f32>();
                let y: f32 = random::<f32>();
                go(iterations - 1, if x.powi(2) + y.powi(2) <= 1_f32 { counter + 1 } else { counter }, to)
            }
        }
    }
    go(iterations, 0, iterations)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monte_carlo_pi_1() {
        let pi = monte_carlo_pi(1);
        assert!(pi == 0_f32 || pi == 4_f32);
    }

    #[test]
    fn monte_carlo_pi_rec_1() {
        let pi = monte_carlo_pi_rec(1);
        assert!(pi == 0_f32 || pi == 4_f32);
    }

    #[test]
    fn monte_carlo_pi_test() {
        let result = monte_carlo_pi(10000_usize);
        println!("iter pi: {}", result);
        assert!(result >= 3_f32 && result <= 4.5_f32)
    }

    #[test]
    fn monte_carlo_pi_rec_test() {
        let result = monte_carlo_pi_rec(10000_usize);
        println!("rec pi: {}", result);
        assert!(result >= 3_f32 && result <= 4.5_f32)
    }

    extern crate test;

    use test::Bencher;

    #[bench]
    fn bench_monte_carlo_pi(b: &mut Bencher) {
        b.iter(|| {
            monte_carlo_pi(10000);
        })
    }

    #[bench]
    fn bench_monte_carlo_pi_rec(b: &mut Bencher) {
        b.iter(|| {
            monte_carlo_pi_rec(10000);
        })
    }
}