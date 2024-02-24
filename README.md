# convertible_couch

Automate Windows display configuration to play from your couch

[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
 ![ci workflow](https://github.com/drsanguin/convertible_couch/actions/workflows/ci.yml/badge.svg)
 [![Coverage Status](https://coveralls.io/repos/github/drsanguin/convertible_couch/badge.svg?branch=main)](https://coveralls.io/github/drsanguin/convertible_couch?branch=main)

## Project Organisation
```mermaid
flowchart TD
bin
common
common_tests
lib

bin --> lib
bin --> common

common_tests --> common

lib --> common
lib --> common_tests
```

## TODO
- Switch audio output, usefull ressources:
  - https://github.com/Belphemur/AudioEndPointLibrary
  - https://github.com/Belphemur/SoundSwitch
  - https://github.com/rust-lang/rust-bindgen
  - https://docs.rs/cpp/latest/cpp/
  - https://github.com/rust-qt/ritual
  - https://github.com/microsoft/windows-rs/issues/480
- Refactor display_settings to reduce code duplication

## Acknowledgments
- [Icon Source](https://www.flaticon.com/free-icon/couch_1010398)
