# API Specification (Initial)

Base URL: (local) http://<server>:8080

Auth: Bearer token (JWT) in Authorization header.

## 1. Auth

### POST /auth/exchange
Request: { code: string }  (session code or QR embedded token)
Response:
{
  token: "jwt",
  user: { id, display_name }
}

## 2. Session

### POST /session
Role: MD only
Request: { code?: string }
Response: { session_id, code }

### GET /session/{id}/snapshot
Response aggregate snapshot (see Data Model).

### POST /session/{id}/close
MD only. Response: { status: "closed" }

## 3. Participants

### POST /session/{id}/join
Headers: Authorization
Body: { }
Response: { status:"pending" | "accepted" }
MD receives WebSocket participant.join_pending event.

### POST /session/{id}/participants/{userId}/approve
MD only
Response: { status:"accepted" }

### DELETE /session/{id}/participants/{userId}
MD or self
Response: { status:"removed" }

## 4. Slots & Roles

### POST /session/{id}/roles/assign
Body: {
  user_id,
  slot_name,
  scope: "global" | "song",
  song_id?
}
Response: { assignment: {...} }

### POST /session/{id}/roles/release
Body: { slot_name, scope, song_id? }

## 5. Setlist

### POST /session/{id}/setlist
Body: { title }
Response: { setlist_id }

### PUT /session/{id}/setlist/items
Body: [
  { song_id, position, key_override?, bpm_override?, song_leader_user_id? }, ...
]

### GET /session/{id}/setlist
Returns: { setlist_id, items:[...] }

### POST /songs
Body: { title, default_key, default_bpm, time_signature, notes? }

## 6. Tempo

### POST /session/{id}/tempo
Body: { bpm, time_signature }
Permissions: MD or delegated click controller.

### POST /session/{id}/tempo/controller
Body: { controller_user_id }

## 7. Metrics

### POST /session/{id}/metrics
Body: { rtt_ms, packet_loss_pct, jitter_ms }

## 8. WebSocket

Path: /ws?token=JWT&session_id=...
Frame Envelope:
{
  type: "participant.join" | "slot.assign" | "tempo.update" | ..., 
  ts: int64 (ms),
  payload: { ... }
}

## 9. Error Format
{
  error: {
    code: "SLOT_CONFLICT" | "UNAUTHORIZED" | "NOT_FOUND" | ..., 
    message: "Human readable"
  }
}

## 10. Rate Limits (initial soft)
- auth/exchange: 5/min per IP
- join: 10/min per IP

## 11. Future Endpoints
- /lyrics
- /recordings
- /mix/advice
