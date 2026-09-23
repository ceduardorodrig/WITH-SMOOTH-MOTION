# with-smooth-motion

Utilitário nativo em Rust para orquestrar execução de jogos com **NVIDIA Smooth Motion** (`VK_LAYER_NV_present`) sob compositores Wayland com scanout direto (Hyprland).

## 🎯 Problema Resolvido

No Hyprland com `render.direct_scanout = 2` (auto):
- Jogos em tela cheia ativam o scanout direto (KMS pageflip direto da GPU para a tela).
- O layer `VK_LAYER_NV_present` da NVIDIA **precisa da composição do compositor** para injetar os frames interpolados na swapchain.
- Sem composição, o frame pacing entra em colapso e corta a taxa de quadros pela metade (**lock a 30fps/36fps**, GPU em ~15%).
- A solução anterior (usar `gamescope`) forçava composição interna, mas introduzia latência de micro-compositor aninhado e gerava **efeito de elástico** (jitter no frametime) quando o VSync in-game estava desligado.

## ⚡ Como Funciona

Este binário em Rust resolve o conflito de forma adaptativa e transparente:
1. **Ativação:** Ao iniciar, executa `hyprctl eval 'hl.config({ render = { direct_scanout = 0 } })'`, forçando a composição do Hyprland durante a sessão do jogo.
2. **Ambiente:** Define `NVPRESENT_ENABLE_SMOOTH_MOTION=1` diretamente no builder do processo filho.
3. **Execução Pura:** Executa o jogo de forma nativa e direta (winewayland puro via Proton).
4. **Restauração via RAII (Drop Trait):** O compilador Rust garante que, ao sair (término normal ou erro), `direct_scanout = 2` é imediatamente restaurado.
5. **Tratamento de Sinais:** Thread dedicada escutando `SIGINT`, `SIGTERM` e `SIGHUP` para restaurar o scanout mesmo se o jogo for forçado a fechar.
6. **Agnóstico:** Se executado fora do Hyprland (ex: KDE Plasma), não altera nada no compositor e apenas passa o Smooth Motion adiante.

## 🛠️ Instalação / Build

```bash
cargo build --release
sudo cp target/release/with-smooth-motion /usr/local/bin/with-smooth-motion
sudo chmod 755 /usr/local/bin/with-smooth-motion
sudo ln -sf /usr/local/bin/with-smooth-motion /usr/bin/with-smooth-motion
```

## 🎮 Como Usar em Novos Jogos (Replicabilidade)

### Caso 1: Jogo Padrão do Steam (Vanilla)
No `~/.config/steam-launch-options/games.toml`, basta definir o perfil como `smooth_motion`:
```toml
[[games]]
appid = 123456
profile = "smooth_motion"
proton = "proton_experimental"
note = "Meu jogo com frame generation NVIDIA nativo"
```
E sincronizar (com Steam fechado):
```bash
steam-launch-options apply 123456
```

### Caso 2: Jogo com Mods / Wrapper Específico (ex: Valheim / r2modman)
No `~/.config/steam-launch-options/profiles.toml`:
```toml
[[profiles]]
name = "r2modman-meujogo"
description = "Mods + Smooth Motion nativo com scanout adaptativo"
options = "PROTON_ENABLE_WAYLAND=1 systemd-run --user --scope game-performance /usr/local/bin/with-smooth-motion \"/caminho/do/wrapper.sh\" %command%"
```

> ⚠️ **Regras de ouro in-game para Smooth Motion:**
> - **V-Sync do jogo:** OFF (evita conflito FIFO com o layer da NVIDIA).
> - **FPS Limiter in-game:** OFF / Unlimited.
