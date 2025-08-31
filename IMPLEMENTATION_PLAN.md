# Implementation Plan

## 1. Guiding Principles
- Vertical slices over broad scaffolding.
- Early measurement of latency & drift.
- Feature flags for experimental pieces (SFU, recording).

## 2. Environment Setup
Week 0:
- Rust workspace (server)
- Flutter project bootstrap
- Shared protobuf/JSON schema module (if we choose codegen later)

## 3. Slices

### Slice A: Session & Auth
- Endpoint: /auth/exchange
- Session create + join approval
- WebSocket baseline (ping/pong, snapshot)

### Slice B: Participants & Slots
- Role assignment endpoints
- Real-time events (slot.assign / release)
- Client UI cards

### Slice C: PTT Audio (Mesh)
- Basic WebRTC signaling over WebSocket
- Mute/unmute logic
- MD priority (client-only attenuation first)

### Slice D: Metronome v1
- Authority selection
- Anchor + beat update frames
- Local click scheduling

### Slice E: Setlist CRUD
- Create, reorder, overrides
- Per-song role override support

### Slice F: Resilience & Metrics
- Reconnect logic
- Metrics reporting & display
- Drift smoothing enhancements

### Slice G: Delegation & Advanced Metronome
- Controller handoff
- Subdivisions
- Drift reports

### Slice H: Accessibility & Polish
- Haptics
- High contrast
- Latency mode switch

### Slice I: Pre-SFU Optimization
- Mesh load warnings
- Bandwidth observation

### Slice J (Optional): Recording Prototype
- Consent model placeholder
- Local mix capture (if feasible)

## 4. Task Breakdown (Example for Slice C)
Server:
- Signaling message handlers
- Peer session registry
Client:
- WebRTC wrapper service
- PTT state store
- Audio permission flow

## 5. Definition of Done (Per Feature)
- Unit tests (server core logic)
- Manual latency sanity check
- Documentation updated
- No critical accessibility regressions

## 6. Tooling
- Lint & format (rustfmt, clippy, dart format)
- CI pipeline (build + minimal tests)
- Future: latency synthetic test harness

## 7. Risks & Mitigations
| Risk | Mitigation |
|------|------------|
| Drift instability | Early instrumentation & anchor smoothing |
| Mesh scaling | Introduce SFU threshold warning |
| Mobile background audio issues | Keep audio session active (platform config) |
| Timer precision variance | Device calibration & fallback adjustments |

## 8. Exit Criteria for Pilot
- <15 ms median drift across 8 devices
- <140 ms 95th percentile PTT voice latency
- Stable reconnect under Wi-Fi blip
- Setlist & role changes propagate reliably

## 9. Future Steps
- SFU integration (separate epic)
- Cross-campus sync protocol