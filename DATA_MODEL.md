# Data Model

## 1. Overview
This document defines the core domain entities, their attributes, and relationships for the rehearsal / live coordination platform.

## 2. Entity Summary
- User
- Session
- Participant (User-in-Session state)
- InstrumentSlot
- Song
- Setlist
- SetlistItem
- RoleAssignment
- TempoState
- MetricsSample

## 3. Entities

### 3.1 User
| Field | Type | Notes |
|-------|------|-------|
| id | UUID | Global unique |
| display_name | string | Human-readable |
| created_at | timestamp | |
| last_active_at | timestamp | Heartbeat update |

### 3.2 Session
| Field | Type | Notes |
|-------|------|-------|
| id | UUID | |
| code | string(6-8) | Human join code or QR |
| md_user_id | UUID | Music Director owner |
| created_at | timestamp | |
| active | bool | Soft close flag |

### 3.3 Participant
Represents a connected User in a Session.
| Field | Type | Notes |
|-------|------|-------|
| session_id | UUID | FK Session |
| user_id | UUID | FK User |
| join_time | timestamp | |
| last_heartbeat | timestamp | |
| ptt_state | enum(idle,talking) | Live voice state |
| network_rtt_ms | int | Last sample |
| packet_loss_pct | float | Last sample |
PRIMARY KEY (session_id, user_id)

### 3.4 InstrumentSlot
Slots are conceptual positions (drums, bass, keys, etc.).
| Field | Type | Notes |
|-------|------|-------|
| id | UUID | |
| name | enum | Predefined list |
| created_at | timestamp | |

### 3.5 RoleAssignment
Mapping of Participant to InstrumentSlot (global or per-song override).
| Field | Type | Notes |
|-------|------|-------|
| session_id | UUID | |
| user_id | UUID | |
| slot_id | UUID | |
| scope_type | enum(global, song) | |
| scope_song_id | UUID? | Present if song scope |
| assigned_at | timestamp | |
UNIQUE (session_id, slot_id, scope_type, scope_song_id)

### 3.6 Song
| Field | Type | Notes |
|-------|------|-------|
| id | UUID | |
| title | string | |
| default_key | string | e.g., “G”, “F#m” |
| default_bpm | int | |
| time_signature | string | “4/4”, “6/8” |
| notes | text | Markdown |

### 3.7 Setlist
| Field | Type | Notes |
|-------|------|-------|
| id | UUID | |
| session_id | UUID | |
| title | string | |
| created_at | timestamp | |

### 3.8 SetlistItem
Orderable reference to Song in a Setlist.
| Field | Type | Notes |
|-------|------|-------|
| setlist_id | UUID | FK |
| position | int | 0-based order |
| song_id | UUID | FK |
| key_override | string? | Optional |
| bpm_override | int? | Optional |
| song_leader_user_id | UUID? | |
PRIMARY KEY (setlist_id, position)

### 3.9 TempoState
Ephemeral authority record.
| Field | Type | Notes |
|-------|------|-------|
| session_id | UUID | |
| bpm | int | |
| time_signature | string | |
| anchor_timestamp | int64 | Unix ms future bar start |
| anchor_bar_index | int | Monotonic |
| controller_user_id | UUID | |

### 3.10 MetricsSample
| Field | Type | Notes |
|-------|------|-------|
| session_id | UUID | |
| user_id | UUID | |
| captured_at | timestamp | |
| rtt_ms | int | |
| packet_loss_pct | float | |
| jitter_ms | int | |

## 4. Relationships
- Session has many Participants.
- Session has one active Setlist (or none).
- Setlist has many SetlistItems (ordered).
- Song can appear multiple times in different Setlists.
- RoleAssignment links Participant ↔ InstrumentSlot optionally per Song.
- TempoState belongs to Session (singleton logical row).
- MetricsSample belongs to Participant (time-series).

## 5. State Snapshots
Initial WebSocket welcome snapshot aggregates:
{
  session: {...},
  participants: [...],
  roleAssignments: [...],
  setlist: {..., items:[...]},
  tempoState: {...}
}

## 6. Derived / Cached Views
- ActiveSongContext: merges Song + overrides + per-song role assignments.
- ParticipantAudioStatus: join of Participant (ptt_state) + slot occupancy.

## 7. Indexing Strategy (Server)
- participants(session_id)
- role_assignment(session_id, scope_type, scope_song_id)
- setlist_item(setlist_id, position)
- tempo_state(session_id) UNIQUE

## 8. Validation Rules
- Only one global assignment per slot per session.
- If per-song assignment exists, it shadows global for that song.
- BPM: 30..260.
- Time signature limited initial: 2/4, 3/4, 4/4, 6/8, 12/8.

## 9. Future Extensions
- LyricsLine / ChordChart entities.
- RecordingSession metadata.
