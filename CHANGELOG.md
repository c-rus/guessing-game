# Changelog

## v1.1.0
### Changes
- distributable package is now a folder containing LICENSE and precompiled binary

## v1.0.6
### Fixes
- checksums are actually computed and stored in file during CI

## v1.0.5
### Changes
- uses python hashlib in CI to compute checksums

## v1.0.4
### Changes
- adds checksum file to releases using sha256

## v1.0.3
### Changes
- releases all binaries in .zip

## v1.0.0
### Features
- implements core game functionality of guessing between a number between 1 and 256
- does not penalize player for an invalid guess input
- provides response to player based on number of guesses it took to discover the number