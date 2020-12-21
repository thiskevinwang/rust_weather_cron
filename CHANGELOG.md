# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.1] - 2020-12-21

### Added

- `./scripts` & `Makefile`
  - use `make deploy`
- aws-cdk code from DynamoDB table
  - table name is `rust-weather-cron` and is shared between TS & Rust files
- CHANGELOG.md

### Changed

- extracted rust code into `lib.rs`

## 0.1.0 - 2020-12-20

### Added

- working POC (locally) of Rust code + AWS-CDK

[0.1.1]: https://github.com/thiskevinwang/rust_weather_cron/compare/v0.1.0...v0.1.1

<!-- [0.1.0]: https://github.com/thiskevinwang/rust_weather_cron/compare/???...v0.1.0 -->
