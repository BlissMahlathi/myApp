# Metronome & Tempo Synchronization Design

## 1. Goals
- Sub-15 ms inter-client audible click alignment.
- Stable perception under network jitter.
- Authority reassignment without tempo jump.

## 2. Authority Model
One controller (MD or delegated user). Only authority sends anchors; others send local drift reports optionally.

## 3. Time Base
Use server monotonic reference OR controller local monotonic clock broadcast via anchor frames:
TempoAnchor {
  bpm: int,
  time_signature: "4/4",
  anchor_bar_index: int,
  anchor_timestamp_ms: int64 (Unix ms future bar start),
  generated_at_ms: int64
}

## 4. Update Frame Types
- tempo.anchor (every N bars or on change)
- tempo.update (lightweight heartbeat each beat)
- tempo.drift.report (client -> authority if abs(drift)>8 ms sustained)

## 5. Scheduling Algorithm (Client)
1. Maintain local estimated start time of current bar: T_bar.
2. For each beat i: scheduled_time = T_bar + i * (60000 / bpm).
3. Use high precision timer (native platform) to fire click ~3–5 ms early, output with short buffer.
4. On receiving anchor:
   - Compute delta = anchor_timestamp_ms - predicted_bar_start.
   - If |delta| < 6 ms: micro-adjust next beats (stretch/shrink few ms).
   - Else: snap at next bar boundary (avoid mid-bar jump).

## 6. Drift Estimation
LocalClockError = received_anchor_timestamp - local_predicted.
Apply exponential moving average (α=0.2) to smooth.

## 7. Jitter Handling
- Maintain circular buffer of last 5 beat intervals; if variance > threshold, request re-anchor.
- If missing 2 consecutive tempo.update frames, enter resync_pending state (UI indicator).

## 8. Delegation Hand-off
Procedure:
1. Current controller sends tempo.handoff { new_controller_user_id, effective_bar_index }.
2. New controller waits for that bar boundary; begins sending anchors from effective_bar_index + 1.

## 9. Audio Rendering
- Use simple high-frequency tick sound (sample or synthesized).
- Provide optional haptic pulse (mobile vibration pattern sub-20 ms).
- Volume ducked under PTT voice only if user enables 'duck click on speech'.

## 10. Precision Considerations
- Avoid relying on audio context currentTime across platforms—use monotonic system clock.
- Compensate output device latency (calibrated once, stored per device).

## 11. Failure Modes
| Scenario | Mitigation |
|----------|------------|
| Controller disconnects | Detect absence of anchor > (bars * duration); auto-elect MD as fallback |
| Wild drift (>40 ms) | Force snap at next bar |
| BPM change mid-bar | Defer until next bar unless urgent flag set |

## 12. Future Enhancements
- Predictive smoothing with network delay modeling.
- Multi-layer pattern (accented beat, subdivision).