# Kinetoxus Roadmap

## Phase 6: Headless Analytical Spring Engine & Retargeting (`kinetocore::spring`) - [completed]
- Analytical closed-form spring solver implemented and proven in `kinetocore`.

## Phase 7: Dioxus Hook Binding (`use_spring`) & Multi-Target Spring Physics - [completed]
- Implemented `use_spring(target, goal, config)` and `Motion::spring` for Dioxus components.
- Supported `SignalTarget<f64>` and `HandleTarget<f64>` with direct memory mutation and zero fake signals.
- Integrated `oxidase::frame` shared frame driver with exact settle clamping and automatic unregister dormancy.
- Proven mid-flight $C^1$ velocity-preserving interruption retargeting.
- Proven dual-host interactive runtime proof on Web (`ego-browser`) and Native (`blitz-host`).

## Phase 8: Multi-Track Timelines & Sequencing (`kinetocore::timeline`) - [upcoming]
- Pure headless timeline engine in `kinetocore`: tracks, keyframes, labels, seek/scrub state machine.
- Dioxus hook binding `use_timeline()` in `kinetoxus`.
