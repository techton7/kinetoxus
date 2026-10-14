# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2026-10-14

### Added
- **Phase 9: Reactive Gesture Continuations (`kinetoxus::gesture`, `use_gesture_continuation`)**:
  - Dioxus hook `use_gesture_continuation` bridging user drag gestures to analytical inertia decay, spring snap-back, and rubberband boundary policies.
  - Reactive `GestureTracker` with configurable tracking windows and exponentially weighted moving velocity calculation.
  - Zero-VDOM and signal target binding modes for high-frame-rate gesture interactions.
- **Phase 10: Composable Timeline Hooks (`use_timeline`)**:
  - Sub-timeline hook builder (`SubTimelineHookBuilder`) enabling hierarchical multi-component animation graphs.
  - Support for `DynamicTimeline` runtime mutation with seamless playhead preservation and active-overlap policies.
  - Qualified `LabelPath` navigation across nested timelines.
- **Dependency Upgrades & Parity**:
  - Upgraded core dependency to `kinetocore 0.3.0` from crates.io.
  - Pinned `oxidase` to release tag `oxidase-v0.1.11` carrying `blitz-host 0.2.0` and Blitz `techton-main-v0.2.3`.

### Changed
- **Feature Semantics**:
  - `blitz-host = ["native", "oxidase/blitz-host"]`: The `blitz-host` feature now explicitly implies `native`, preserving compatibility with documented testing CLI commands while allowing production consumers to use pure `native` without test IPC harnesses.
  - Moved `web-time = "1.1"` to `[dev-dependencies]`.

## [0.1.0](https://github.com/techton7/kinetoxus/releases/tag/v0.1.0) - 2026-09-15

### Added

- initial commit for kinetoxus
