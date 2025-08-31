# Audio Architecture

## 1. Objectives
Deliver low-latency, intelligible comms with minimal feedback and consistent tempo cues.

## 2. Stack Layers
- Capture: Platform mic (AEC, NS, AGC enabled)
- Encoding: Opus (mono 24 kbps target, variable)
- Transport: WebRTC SRTP
- Playback Mix: Remote peer streams + locally synthesized metronome
- Output Chain: Gain staging → Optional ducking → Device

## 3. Capture Settings
| Parameter | Value |
|----------|-------|
| Sample Rate | 48 kHz |
| Channels | 1 (mono) |
| Frame Size | 20 ms (Opus) |
| Bitrate | 16–32 kbps adaptive |

## 4. Push-To-Talk Flow
User presses → local mute flag cleared → immediate RTP frames start (no gating delay). On release, send silent comfort noise or pause stream (depending on renegotiation overhead strategy; initial: keep stream, just mute track).

## 5. Ducking
When MD speaks:
- Non-MD streams attenuated -6 dB (client side) OR
- Optional "focus mode" where only MD remains full volume.

## 6. Latency Considerations
- Avoid large jitter buffers (target 60–80 ms).
- Allow manual 'low-latency' toggle vs 'stable' mode.

## 7. Metronome Integration
Metronome audio rendered locally (synth or short sampled click) mixed pre-output, independent of WebRTC pipeline (no extra network load).

## 8. Echo & Feedback Prevention
Assumption: Headset use. Still enforce AEC where platform provides.

## 9. Failure Handling
If packet loss > 10% sustained for 5s:
- Display network warning
- Optionally reduce bitrate floor

## 10. Future
- Spatialization (pan by instrument)
- Automatic loudness normalization (EBU R128 style simplified)
- Music-mode high-fidelity channel (for instrument showcase)
