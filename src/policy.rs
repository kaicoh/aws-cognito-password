use crate::distributions::*;
use rand::seq::SliceRandom;
use rand::Rng;

// MEMO: https://docs.aws.amazon.com/cognito/latest/developerguide/user-pool-settings-policies.html
const MIN_LENGTH: u8 = 6;

#[derive(Debug, Copy, Clone)]
pub struct PasswordPolicy {
    length: u8,
    require_number: bool,
    require_special: bool,
    require_upper: bool,
    require_lower: bool,
}

impl Default for PasswordPolicy {
    fn default() -> Self {
        Self {
            length: 8,
            require_number: true,
            require_special: true,
            require_upper: true,
            require_lower: true,
        }
    }
}

impl PasswordPolicy {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_length(self, length: u8) -> Self {
        let value = if length < MIN_LENGTH {
            MIN_LENGTH
        } else {
            length
        };

        Self {
            length: value,
            ..self
        }
    }

    pub fn contains_at_least_1_number(self, value: bool) -> Self {
        Self {
            require_number: value,
            ..self
        }
    }

    pub fn contains_at_least_1_special_character(self, value: bool) -> Self {
        Self {
            require_special: value,
            ..self
        }
    }

    pub fn contains_at_least_1_uppercase_letter(self, value: bool) -> Self {
        Self {
            require_upper: value,
            ..self
        }
    }

    pub fn contains_at_least_1_lowercase_letter(self, value: bool) -> Self {
        Self {
            require_lower: value,
            ..self
        }
    }

    pub fn gen(&self) -> String {
        self.gen_with_rng(&mut rand::thread_rng())
    }

    pub fn gen_with_rng<R: Rng + ?Sized>(&self, rng: &mut R) -> String {
        let mut chars: Vec<char> = (0..self.length)
            .map(|_| rng.sample(AnyLetter) as char)
            .collect();

        if self.require_upper {
            chars[0] = rng.sample(UppercaseLetter) as char;
        }

        if self.require_lower {
            chars[1] = rng.sample(LowercaseLetter) as char;
        }

        if self.require_number {
            chars[2] = rng.sample(Number) as char;
        }

        if self.require_special {
            chars[3] = rng.sample(SpecialCharacter) as char;
        }

        chars.shuffle(rng);
        chars.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::RngCore;

    fn test_rng(seed: u64) -> impl RngCore {
        const INC: u64 = 11634580027462260723;
        rand_pcg::Pcg32::new(seed, INC)
    }

    #[test]
    fn it_build_password_policy_with_default_values() {
        let policy = PasswordPolicy::new();
        assert_eq!(policy.length, 8);
        assert!(policy.require_number);
        assert!(policy.require_special);
        assert!(policy.require_upper);
        assert!(policy.require_lower);
    }

    #[test]
    fn it_sets_length() {
        let policy = PasswordPolicy::new().set_length(16);
        assert_eq!(policy.length, 16);
    }

    #[test]
    fn it_sets_six_if_length_is_smaller_than_six() {
        let policy = PasswordPolicy::new().set_length(5);
        assert_eq!(policy.length, 6);
    }

    #[test]
    fn it_sets_require_at_least_1_number_flag() {
        let policy = PasswordPolicy::new().contains_at_least_1_number(false);
        assert!(!policy.require_number);
    }

    #[test]
    fn it_sets_require_at_least_1_special_character_flag() {
        let policy = PasswordPolicy::new().contains_at_least_1_special_character(false);
        assert!(!policy.require_special);
    }

    #[test]
    fn it_sets_require_at_least_1_uppercase_letter_flag() {
        let policy = PasswordPolicy::new().contains_at_least_1_uppercase_letter(false);
        assert!(!policy.require_upper);
    }

    #[test]
    fn it_sets_require_at_least_1_lowercase_letter_flag() {
        let policy = PasswordPolicy::new().contains_at_least_1_lowercase_letter(false);
        assert!(!policy.require_lower);
    }

    #[test]
    fn it_generates_password_from_given_rng() {
        let mut rng = test_rng(123);
        let policy = PasswordPolicy::new();
        let password = policy.gen_with_rng(&mut rng);
        assert_eq!(password, "c`;X0Hpv");
    }
}
