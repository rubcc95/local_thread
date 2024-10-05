# Changelog

## [0.1.1] - 2024-10-05
### Added
- `FnOnce` types have been wrapped in `ManuallyDrop` to prevent double free undefined behavior for types that implement both `FnOnce` and `Drop`. In future versions of Rust, `FnOnce` could be implemented by a `Drop` type. 

## [0.1.0] - 2024-10-01
### Initial Commit
- Initial release of the project.
- Basic functionality implemented.