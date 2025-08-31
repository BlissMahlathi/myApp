# WebRTC Evaluation

## 1. Why WebRTC
- Built-in NAT traversal (even if limited here).
- Opus + congestion control.
- Widely supported mobile implementations.
- Security (DTLS-SRTP) out-of-box.

## 2. Mesh vs SFU
| Aspect | Mesh | SFU |
|--------|------|-----|
| Upstream bandwidth | N-1 per sender | 1 per sender |
| Complexity | Lower | Higher |
| Latency | Direct peer; minimal | One extra hop (~5–20 ms) |
| Scaling practical | ~12–15 peers | 50+ with tuning |

Initial: Mesh, pivot to SFU once pain emerges.

## 3. ICE Constraints
LAN environment: prefer host candidates.
Disable TURN requirement initially; fallback future.

## 4. Codec Choices
Audio: Opus
Reason: Low latency, adaptive, robust to loss.

## 5. Media Topology Roadmap
Phase 1: Full mesh
Phase 2: Introduce Rust SFU (selective forward)
Phase 3: Cross-campus bridging (federated SFUs)

## 6. Risk Analysis
- Mesh bandwidth blowup at higher counts.
- Mobile CPU for many decoders.
Mitigation: early metrics triggers (warn MD at 12 peers).

## 7. Alternative Considered
Custom UDP audio:
+ Potentially lower overhead
- Reinvent congestion, encryption, device integration.

Decision: Stick with WebRTC core.

## 8. Future Enhancements
- RED / FEC for loss resilience.
- Simulcast low bitrate fallbacks (if SFU).