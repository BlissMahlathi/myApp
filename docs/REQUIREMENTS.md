# Requirements Specification

## 1. Stakeholders
| Stakeholder | Needs |
|-------------|-------|
| Music Director (MD) | Session authority, assign roles, control metronome, enforce mutes |
| Drummer | Follow & sometimes control click if permitted (tempo, start/stop) |
| Instrumentalists (Keys 1, Keys 2, Bass, Guitars, Sax, etc.) | Hear team, get song context (key/BPM/progression), communicate discreetly |
| Vocalists (Lead, Backing) | Song order, lyrics integration (future), communication |
| Song Leader (Per-song) | Drive flow cues, may not be MD |
| Tech Engineer | Receive targeted instructions (e.g., “raise lead vocal in mains”) |
| Guest / Observer | Limited listen-only or data-only role |
| Click Controller (logical role) | Might be MD or Drummer; ensures consistent tempo authority |

## 2. Role Model
CoreRole enum:
- MD
- TECH
- MUSICIAN
- GUEST

InstrumentSlot enum (assignable globally & per-song override):
- lead_vocal
- backing_vocal_1
- backing_vocal_2
- backing_vocal_3
- song_leader
- keys_primary
- keys_secondary
- bass
- electric_guitar
- acoustic_guitar
- drums
- sax
- percussion_aux
- tech_engineer
- click_controller
- unassigned

Per-song overrides: `song_leader`, `lead_vocal`, and any slot needing change.

## 3. Functional Requirements

3.1 Session Management  
- MD hosts; participants join via PIN/QR.  
- Approval queue (MD accepts/denies).  
- On join, suggested slot based on historical usage.

3.2 Audio Communication  
- Push-to-talk (default).  
- MD priority override (others duck).  
- Optional toggle mode (later).  

3.3 Instrument Slot & Song Role Assignment  
- MD assigns global & per-song.  
- Conflict detection (one occupant per slot per scope).  

3.4 Metronome  
- Central authority (MD or delegated click_controller).  
- Broadcast tempo, time signature, future bar timestamp.  
- Drift correction < 15 ms target deviation.

3.5 Setlists  
- Create/edit order.  
- Song instance carries key, BPM, arrangement notes, leader overrides.  

3.6 Sync  
- WebSocket + DataChannel for events: slots, roles, tempo, priority talk.

3.7 Security  
- Auth token with role claims.  
- MD-only mutation endpoints.  

3.8 Device / Headset Handling  
- Mic availability check.  
- Packet loss & RTT indicator.

3.9 Accessibility  
- Large touch zones.  
- Color-blind safe palette.  
- Haptic beat (optional).

## 4. Non-Functional
| Category | Requirement |
|----------|-------------|
| Latency | <120 ms one-way speech |
| Metronome Drift | <15 ms among clients |
| Capacity | 15 typical, up to 25 |
| Reliability | Graceful reconnection < 5 s |
| Persistence | SQLite local; resilient to crash |
| Observability | Metrics for RTT, drift, packet loss |

## 5. Constraints
- Offline LAN (no external STUN/TURN).
- Rust backend (Tokio).
- Flutter mobile apps.
- WebRTC chosen for low-latency audio.

## 6. Assumptions
- All participants wear headsets; echo minimized.
- Stable power during service.

## 7. Open Questions
- Lyric integration priority?  
- Multi-campus bridging future?  
- Recording legality & consent model?  
- Should Tech have silent private channel with MD early?