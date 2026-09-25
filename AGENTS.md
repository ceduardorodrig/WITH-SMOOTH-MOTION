---
tags: [meta, agents, governance, linux, wayland]
---

# AGENTS.md — WITH-SMOOTH-MOTION Governance Rules

This repository contains **WITH-SMOOTH-MOTION**, an adaptive direct scanout and tearing governor for Wayland (Hyprland / Valve games) written in pure Rust.

When modifying any file in this repository, follow these mandatory governance rules:

**Language Tier:** A (Public OSS) — see [language-policy.md](file:///mnt/NVME_PCI/agentic-ai/governance/language-policy.md). All logs, CLI strings, documentation, and comments MUST be in English.

## 🦀 Rust Standards & Integrity

1. **NATIVE RUST & SOVEREIGNTY (RUST 2024)** — All code must remain in native compiled Rust (`edition = "2024"`). Intermediate scripts are strictly prohibited (`ARCH-NO-PYTHON`).

2. **PROHIBITION OF UNWRAP/EXPECT IN PRODUCTION (`RUST-NO-UNWRAP`)** — Error handling must be deterministic using `?`, `match`, or safe fallbacks (`unwrap_or`). Using `unwrap()` or `expect()` in production triggers panics and is classified as a bypass attempt.

3. **MANDATORY STENIOSENTINEL VERIFICATION (RULE 0)** — Before completing any turn or committing, execute `stenio --path .`. The Quality Gate must pass with zero blocking errors.

4. **ZERO CREDENTIALS & SECRETS (`SEC-SECRETS`)** — Never commit credentials, tokens, or private secrets.

5. **STANDARDIZED README DISCLAIMER** — The root `README.md` must preserve the standardized vibe-coded governance disclaimer:
   ```markdown
   <div align="center">

   > **Yes... This is a Vibe Coded project**
   >
   > Governed by 🤖 **StenioSentinel** (our Rust-based AI Governance Sentinel) with **Carlos Eduardo Rodrigues** ([@ceduardorodrig](https://github.com/ceduardorodrig)).

   </div>
   ```
