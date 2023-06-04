# aws-cognito-password

[![Version](https://img.shields.io/crates/v/aws-cognito-password)](https://crates.io/crates/aws-cognito-password)
[![License](https://img.shields.io/crates/l/aws-cognito-password)](LICENSE)
[![Test](https://img.shields.io/github/actions/workflow/status/kaicoh/aws-cognito-password/test.yml)](https://github.com/kaicoh/aws-cognito-password/actions/workflows/test.yml)

A password generator for aws cognito userpool.

```rust
use aws_cognito_password::PasswordPolicy;

let policy = PasswordPolicy::new();
let password = policy.gen();

println!("password: {}", password);
```

## License

This software is released under the [MIT License](LICENSE).
