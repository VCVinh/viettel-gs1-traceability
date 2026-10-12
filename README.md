# 🇻🇳 Viettel × GS1 Digital Link & Offline-First Rust P2P Traceability Platform (v3.0)

**Standalone Architecture & Production Implementation** for the **Viettel Partnership (Stages 0 → 3)**:
- **Stack 1 (`Gs1UriResolverEngine`):** GS1 Digital Link 1.7 + IETF RFC 9264 (`application/linkset+json`) + Modulo-10 Check Digit + Universal Links (`apple-app-site-association` / `assetlinks.json`) + Viettel CAMARA 5G Base-Station Cell-ID Proof-of-Scan (`10,000+ RPS`, `<20ms` SLA).
- **Stack 2 (`RustP2pMobileCore`):** Offline-First Rust P2P & CRDT Mobile Core (`BinaryFuse16`, `RoaringBitmap`, `Loro/Iroh` CRDTs, mDNS/BLE local mesh, `<0.1ms` offline CPU verification in Airplane Mode with `<30MB` RAM footprint, C-FFI exports).
- **Stack 3 (`ConsumerMobileAppSimulator` & `ViettelEcosystemPlatformEngine`):** Step 1 Online Green Blockchain Badge → Step 2 Airplane Mode Offline Orange P2P Badge, 60-Second SME 10,000-Barcode Onboarding (`Thông tư 02/2024/TT-BKHCN`), `+500MB 5G` Loyalty Rewards, and Viettel Post AR MatrixScan Anti-Swap Waybill Verification.
- **Stack 4 (`HyperledgerPolygonLedgerAnchor` & Holographic Truth v3.0):**
  - **Solidity EAS Attestation Contract:** [`contracts/ViettelTraceabilityEAS.sol`](contracts/ViettelTraceabilityEAS.sol)
  - **Non-Commutative GHRR Quaternion Wave-CRDT (`HolographicWaveState`):** $SU(2) \cong S^3$ causal hop ordering & $O(D \log D)$ wave interference CRDT.
  - **Cellular Sheaf Cohomology $H^1(\mathcal{G}; \mathcal{F})$ (`SupplyChainCellularSheaf`):** Dirichlet energy & Yang-Mills gauge holonomy detector catching 1-for-1 counterfeit substitutions at distributor hubs before consumer scans.
  - **Through-Container Bessel Acoustic Verification (`ThroughBottleAcousticVerification`):** 18–22 kHz smartphone ultrasonic chirp verifying sealed liquid interior speed of sound and viscosity $Q$-factor.

---

## 📄 Technical Dossiers (PDF & Markdown)

- [`docs-pdf/viettel_gs1_p2p_ai_employees_autopilot_dossier.pdf`](docs-pdf/viettel_gs1_p2p_ai_employees_autopilot_dossier.pdf) — Stage 0 → 3 Execution Blueprint & 4-Stack Architecture
- [`docs-pdf/planetary_traceability_frontier_sota_dossier.pdf`](docs-pdf/planetary_traceability_frontier_sota_dossier.pdf) — Global Regulatory Mandates (GS1 Sunrise 2027, EU DPP, FDA FSMA 204, Thông tư 02/2024/TT-BKHCN) & Commercial Benchmarks
- [`docs-pdf/critical_system_review_and_best_architecture.pdf`](docs-pdf/critical_system_review_and_best_architecture.pdf) — Critical System Review: Rust vs. Solidity & Optimal Hybrid Stack
- [`docs-pdf/truth_vs_friction_holographic_matter_state_dossier.pdf`](docs-pdf/truth_vs_friction_holographic_matter_state_dossier.pdf) — Truth Over Friction: Wave-CRDTs, Sheaf Cohomology $H^1$, Folding PCD & Matter Identity

---

## 🚀 Quick Start

```bash
# Run all unit tests (GS1 Resolver, Offline P2P CRDT, GHRR Quaternions, Sheaf Cohomology H^1, Bessel Acoustics)
cargo test

# Generate the live Viettel Demo HTML & JSON artifacts in demo-output/
cargo run -- demo

# Start the standalone Viettel GS1 HTTP Server on http://127.0.0.1:8090
cargo run -- serve
```
