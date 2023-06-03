use rand::distributions::{Distribution, Uniform};
use rand::Rng;

pub struct UppercaseLetter;
pub struct LowercaseLetter;
pub struct Number;
pub struct SpecialCharacter;
pub struct AnyLetter;

const UPPERCASES: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWERCASES: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const NUMBERS: &[u8] = b"0123456789";
const SPECIALS: &[u8] = b"^$*.[]{}()?\"!@#%&/\\,><':;|_~`=+-";

impl Distribution<u8> for UppercaseLetter {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> u8 {
        let index = Uniform::new(0, 26).sample(rng) as usize;
        UPPERCASES[index]
    }
}

impl Distribution<u8> for LowercaseLetter {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> u8 {
        let index = Uniform::new(0, 26).sample(rng) as usize;
        LOWERCASES[index]
    }
}

impl Distribution<u8> for Number {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> u8 {
        let index = Uniform::new(0, 10).sample(rng) as usize;
        NUMBERS[index]
    }
}

impl Distribution<u8> for SpecialCharacter {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> u8 {
        let index = Uniform::new(0, 32).sample(rng) as usize;
        SPECIALS[index]
    }
}

impl Distribution<u8> for AnyLetter {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> u8 {
        let range = 26 + 26 + 10 + 32;
        let index = Uniform::new(0, range).sample(rng) as usize;
        [UPPERCASES, LOWERCASES, NUMBERS, SPECIALS].concat()[index]
    }
}
