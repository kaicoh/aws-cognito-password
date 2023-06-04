//! # aws cognito password
//!
//! This crate supports generating passwords for aws cognito userpool.
//!
//! ```
//! use aws_cognito_password::PasswordPolicy;
//!
//! let policy = PasswordPolicy::new();
//! let password = policy.gen();
//!
//! println!("password: {}", password);
//! ```
mod distributions;
mod policy;

pub use policy::PasswordPolicy;
