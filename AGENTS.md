---
tags: [meta, agents, governance, linux, wayland]
---

# AGENTS.md — WITH-SMOOTH-MOTION Governance Rules

Este repositório contém a ferramenta **WITH-SMOOTH-MOTION**, um gerenciador adaptativo de direct scanout e tearing para Wayland (Hyprland / Valve games) escrito em Rust nativo.

Ao modificar qualquer arquivo deste repositório, siga estas regras obrigatórias de governança:

## 🦀 Padrões Rust & Integridade

1. **RUST NATIVO & SOBERANIA (RUST 2024)** — Todo código deve permanecer em Rust compilado nativo (`edition = "2024"`). Proibido scripts intermediários.

2. **PROIBIÇÃO DE UNWRAP/EXPECT EM PRODUÇÃO (`RUST-NO-UNWRAP`)** — Tratamento de erros deve ser determinístico usando `?`, `match` ou fallback seguro (`unwrap_or`).

3. **VERIFICAÇÃO OBRIGATÓRIA DO STÊNIOSENTINEL (REGRA 0)** — Antes de qualquer commit, é obrigatório executar `stenio --path .`. O Quality Gate deve aprovar com zero erros bloqueantes.

4. **SEGURANÇA & ZERO SEGREDOS (`SEC-SECRETS`)** — Nenhuma credencial ou token deve ser adicionado ao código.

5. **DISCLAIMER PADRONIZADO NO README** — O `README.md` raiz deve manter o disclaimer padronizado de governança humana-IA:
   ```markdown
   <div align="center">

   > **Yes... This is a Vibe Coded project**
   >
   > Governed by 🤖 **StenioSentinel** (our Rust-based AI Governance Sentinel) with **Carlos Eduardo Rodrigues** ([@ceduardorodrig](https://github.com/ceduardorodrig)).

   </div>
   ```
