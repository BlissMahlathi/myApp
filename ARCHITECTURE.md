# System Architecture

## 1. Overview
This document describes the end-to-end architecture of the rehearsal / live service coordination platform. Goals: ultra-low-latency team communication, authoritative tempo (metronome) sync, structured musical context (setlists, keys, roles), and robust offline-friendly operation on a local network without external TURN.

## 2. High-Level Component Map
- Mobile Client (Flutter)
  - UI layer (Screens, State Controllers)
  - Data layer (Repository pattern; WebSocket + REST adapters)
  - Audio layer (WebRTC wrapper + PTT logic + metronome rendering)
  - Local persistence (SQLite via drift/moor)
- Control & Signaling Server (Rust / Tokio)
  - Auth & Session Management
  - Role & Slot Assignment Service
  - Setlist & Song Metadata Service
  - Tempo Authority (metronome broadcaster) coordination
  - WebSocket Hub (Pub/Sub style event fanout)
  - Metrics & Health (Prometheus style exporters future)
- WebRTC Transport (Peer <-> Peer with Server-assisted signaling)
  - ICE negotiation (host / srflx only due to LAN constraint)
  - Optional future SFU module (Rust-based) for scaling > ~15 participants
- Persistence Layer
  - Ephemeral in-memory session state + periodic snapshot to disk (server)
  - SQLite/ Postgres future (for multi-campus)
- Observability & Telemetry
  - Client metrics (RTT, packet loss) -> periodic report messages
  - Server log + structured events -> future export

## 3. Data Flows
### 3.1 Session Join
1. Client obtains token (pre-shared / QR) -> REST /auth/exchange
2. Opens WebSocket -> /ws?token=...
3. Receives initial snapshot: participants, slots, setlist, current tempo state
4. Begins ICE signaling (offer/answer) through /ws channel

### 3.2 Metronome Sync
Authority sends TempoFrame every beat (plus periodic future anchor w/ absolute start time). Clients compute drift and adjust scheduled clicks.

### 3.3 Push-To-Talk (PTT) Voice
Client UI press -> unmute track & send RTP. Release -> mute & send state update (to enable ducking logic).

## 4. Event Model
Single WebSocket multiplexes typed JSON frames:
- participant.join / participant.leave
- slot.assign / slot.release
- tempo.update (lightweight) / tempo.anchor (full future bar timestamp)
- ptt.state (talking, idle)
- setlist.update / song.current
- metrics.report

## 5. Performance Targets
- Audio RTT: <120 ms one-way
- Metronome inter-client drift: <15 ms
- Reconnect recovery: <5 s

## 6. Scaling Strategy
Phase 1: Mesh WebRTC (each peer to each) — practical ~12-15.
Phase 2: Introduce SFU to centralize upstream per speaker.
Phase 3: Multi-campus bridging via selective forwarding + tempo relay.

## 7. Security
- Short-lived session token w/ role claims (JWT HS256 local secret)
- MD-only privileged mutations validated server-side
- Rate limiting join attempts

## 8. Failure Recovery
- Client auto-retry WebSocket (exponential 0.5 -> 5s)
- If tempo frames missed > 2 anchors, trigger resync request
- ICE restart if media stalled > 3s

## 9. Technology Choices (Rationale)
- Rust: safety & async performance for low-latency event fanout
- Flutter: unified mobile delivery & rapid UI iteration
- WebRTC: built-in congestion control + Opus + echo cancellation

## 10. Future Extensions
- Recording with consent ledger
- Lyrics / chord chart overlay
- AI-driven mix suggestions
