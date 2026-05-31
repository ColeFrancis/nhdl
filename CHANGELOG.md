# Change Log
All notable changes to this project will be documented in this file.
 
The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

### Changed

- Events happening in the same time step are handled in a random order
- Run takes as a first argument the max number of time steps it will be allowed to take.
- Run returns the number of steps it took as an option, none if max number of steps was exceeded
- Run takes as a second argument whether only necessary events are handled or not.

### Added

- Added watchers

## [0.1.0] - 2026-05-30

### Added

- Created project directory structure
- Created core logic simulator