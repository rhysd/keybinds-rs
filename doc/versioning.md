Versioning
==========

This crate conforms to [Semantic Versioning 2.0.0][semver], but it has not reached 1.0.0 yet (although the API is stable
enough for production use). The versioning rule of this crate is as follows.

- **Major** is fixed to `0`.
- **Minor** is bumped when some breaking change is introduced for example:
  - API breaking changes
  - Major version bumps in the optional dependencies
  - MSRV bump
- **Patch** is bumped when some compatible changes are added for example:
  - New features such as adding new framework/library support
  - Fixes

[semver]: https://semver.org/
