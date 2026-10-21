# Changelog

All notable changes to this project are documented in this file. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the project uses
[Semantic Versioning](https://semver.org/).

## [Unreleased]

### Added

- Project bootstrap: repository layout, build system, development environment, CI and
  documentation.
- `keyway-engine`: a D-Bus-activated GIO service implementing GNOME Shell's
  `SearchProvider2`, answering with a single "Keyway is installed" result.
- `keywayctl doctor`: checks that the engine is installed and answers.
