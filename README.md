# Worship Team Communication & MD Control App

A local-network (hotspot) powered system enabling a Music Director (MD) and all musicians (instrumentalists, vocalists, technicians) to:
- Communicate in real time through in‑ear headsets (earphones/headphones with mic) using low-latency audio
- Share dynamic setlists (keys, chord progressions, arrangement notes)
- Run a dynamic, authoritative metronome synchronized across devices
- Assign per-song roles (e.g., Lead Vocal changes between songs, Keys 1 vs Keys 2, etc.)
- Control mutes, volumes, and priority talk (MD override)
- Operate completely offline during rehearsal/service

> Status: Design Documentation (Implementation not yet present)

## Core Value
Unifies communication, tempo, and musical context so the team performs tighter transitions, consistent timing, and maintains professional coordination without shouting or hand signals.

## Highlight Features
- Offline hotspot session hosting by MD laptop
- Rust backend for performance (signaling, data authority, audio routing control)
- Flutter mobile clients (Android/iOS) as musician endpoints
- Real-time audio via WebRTC (Opus codec)
- Per-song dynamic role assignments (Song Leader, Lead Vocal, instrument slots)
- MD & (optionally) Drummer can control metronome tempo and start/stop
- Setlist-aware tempo & key display
- Click + visual pulse + optional haptic feedback
- Local persistence (SQLite) and export/import of setlists
- Scoped permissions (MD, Tech, Musician, Guest) plus instrument slots

## Rust Backend Rationale
- Predictable performance for real-time signaling and timing
- Efficient concurrency (Tokio) for handling multiple WebRTC negotiations
- Strong typing for minimizing runtime errors in critical audio/timing logic

## Roles & Instrument Slots (Concept)
Two layers:
1. Core Role: MD, TECH, MUSICIAN, GUEST
2. Instrument/Vocal Slots (Assignable per song & globally):
   - lead_vocal
   - backing_vocal_1, backing_vocal_2, backing_vocal_3
   - keys_primary (Piano / Keys 1)
   - keys_secondary (Pads / Keys 2)
   - bass
   - electric_guitar
   - acoustic_guitar
   - sax
   - drums
   - percussion_aux
   - tech_engineer
   - click_controller (can map to MD or Drummer)
   - song_leader (distinct: drives transitions; may coincide with lead_vocal)

Per-song overrides allow “song_leader” and “lead_vocal” to shift without changing base participant identity.

## MVP Scope (Refined)
- Session hosting (MD)
- Join via PIN/QR, approval workflow
- Audio PTT (push-to-talk) with MD priority override
- Per-user mute (by MD) and local volume control
- Dynamic instrument slot assignment; per-song leader designation
- Metronome broadcast + drift correction
- Setlist CRUD; song instances include tempo/key/progression
- Persist to SQLite; JSON export
- Minimal UI for Tech to receive targeted communication

## Non-Goals (Early)
- Personalized multi-mix with full DSP
- Cloud synchronization
- Automatic chord transposition rendering (beyond Roman numerals/basic key adaptation)
- Multi-campus linking

## Hardware Assumptions
- Each participant uses a mobile device + wired/wireless headset with integrated mic
- Stage noise manageable; encourage closed-back or in-ear monitors
- Drummer may need stronger click accent (client-level gain for metronome channel)

## Repository Layout (Planned)
```
/docs
  REQUIREMENTS.md
  ARCHITECTURE.md
  DATA_MODEL.md
  API_SPEC.md
  METRONOME_DESIGN.md
  ROADMAP.md
  AUDIO_ARCHITECTURE.md
  WEBRTC_EVALUATION.md
  BRANDING_GUIDE.md
  UX_WIREFRAMES.md
  IMPLEMENTATION_PLAN.md
/server-rust
/mobile
/md-ui
/proto
/scripts
```

## License / Contribution
(To be defined)