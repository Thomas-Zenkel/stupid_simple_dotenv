# Changelog

All notable changes to this project will be documented in this file.

## [0.3.0] - 2025-01-04

### Breaking Changes

- **`to_env()` and `file_to_env()` no longer override existing environment variables.**
  This aligns with the standard dotenv behavior where shell exports take precedence over `.env` file values.
  If you relied on the old behavior of overriding existing variables, use the new `to_env_override()` or `file_to_env_override()` functions instead.

### Added

- `to_env_override()` - Reads `.env` file and sets environment variables, overriding existing values
- `file_to_env_override()` - Reads a custom file and sets environment variables, overriding existing values

### Changed

- `to_env()` now only sets variables that are not already present in the environment
- `file_to_env()` now only sets variables that are not already present in the environment

## [0.2.5] - Previous release

- Bug fixes and improvements
