# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.1](https://github.com/techton7/kinetoxus/compare/v0.1.0...v0.1.1) - 2026-10-08

### Added

- *(playground)* unify examples into Tailwind CSS motion playground
- *(showcase)* implement timeline_showcase and dual-host interactive verification (T-13.6)
- *(showcase)* author timeline_showcase example and align test suite with chained TrackBuilder ergonomics
- *(timeline)* implement use_timeline hook, TimelineController, and real-time scrubber (T-13.4)
- *(spring)* implement scalar use_spring target binding with frame-loop lifecycle and dual-target proof
- *(kinetoxus)* implement non-signal HandleTarget substrate and first handle proof
- *(kinetoxus)* add to/from motion verbs
- *(driver)* integrate unified oxidase frame driver and adopt high-level DX
- *(kinetoxus)* add phase-1 dioxus signal mvp
- depend on and re-export kinetocore

### Fixed

- *(playground)* wire frame subscription and rich transforms to HandlesSection
- *(playground)* add bg-no-repeat to all gradient cards to eliminate discrete tiling seams
- *(kinetoxus)* exercise motion.set in reset and align action status semantics

### Other

- *(deps)* update blitz to techton-main-v0.2.2 and add bg-no-repeat to gradient cards
- *(playground)* modularize playground into section components
- *(deps)* use kinetocore v0.2.0 from crates.io and remove local path patch
- *(deps)* update blitz to techton-main-v0.2.1
- add Phase 8 multi-channel visual choreography contract test matrix
- *(deps)* update kinetocore to v0.2.0 git tag
- *(deps)* update Blitz dependencies from local paths to techton-main-v0.2.0 git tag
- *(kinetoxus)* record Milestone T-7 dogfooding test matrix and path resolution proof
- *(kinetoxus)* record T-5 graphics optimization and scaling band root cause analysis
- *(kinetoxus)* prove phase 4 dual-target interactive showcase
- *(kinetoxus)* prove signal demo on blitz-host lane
- upgrade oxidase to v0.1.7 with blitz-host v0.1.2 and expose native/blitz-host features
- *(kinetoxus)* consume oxidase frame v0.1.4
- *(kinetoxus)* record tracking issue for native frame driver (ISSUE-0001)
- release v0.1.0

## [0.1.0](https://github.com/techton7/kinetoxus/releases/tag/v0.1.0) - 2026-09-15

### Added

- initial commit for kinetoxus
