# UX Wireframes (Textual Description)

(Visual sketches to be added later; this textual spec guides initial Flutter layouts.)

## 1. Main Screens
1. Join Session
2. Lobby / Approval Pending
3. Rehearsal Dashboard
4. Setlist Manager
5. Metronome Control Panel
6. Slot Assignment Overlay
7. Network / Metrics Panel
8. Settings & Accessibility

## 2. Rehearsal Dashboard Layout
Top:
- Current Song Title • Key • BPM • Time Signature
- Next Song (preview chip)

Center Grid:
- Role cards (Drums, Bass, Keys, etc.) showing occupant avatar or “Open”.
- Active speaker glow ring.

Bottom Bar:
- PTT button (large), Metronome toggle (if controller), Setlist button, Metrics.

Floating Action (MD only):
- Assign Slots
- Tempo Settings

## 3. PTT Button States
- Idle: Solid primary
- Pressed: Bright accent ring
- MD Speaking (others view): Bar at top “MD Talking…”

## 4. Slot Assignment Overlay
List of slots with current occupant; tapping empty prompts assignment. Conflict warnings inline.

## 5. Setlist Manager
Reorderable list:
[Drag Handle] Song Title  (Key / BPM overrides tag)
Add Song button -> dialog search/create.

## 6. Tempo Panel
Fields: BPM (spinner), Time Signature (picker), Controller (dropdown), “Send Anchor Now”.

## 7. Metrics Panel
- RTT gauge
- Packet Loss %
- Drift (ms) (if clicked controller)
- Peer count & mesh load warning badge.

## 8. Accessibility
- High contrast toggle
- Haptic metronome switch
- Larger PTT size option

## 9. Error / Alert Patterns
Toast (3s): minor confirmations.
Modal: destructive or role authority changes.
Inline banner: network degradation.

## 10. Future
- Gesture shortcuts
- Quick role swap action