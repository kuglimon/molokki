# Molokki

Monorepo for all my personal projects.

I got bored of tracking all my unfinished projects across multiple repos and
often I didn't even bother pushing them to Github. Hence molokki was born to
have one repo to store all code.

Molokki is a Finnish word for a garbage can that extends underground.

## Projects

```sh
.
├── aoc2023              # advent of code 2023 puzzles
├── pipemixer            # terminal based pipewire mixer
├── fallout-save-editor  # for editing Fallout 2 save files
├── deus-ex-iw-patcher   # fixes (like FOV) for DX:IW
├── kube-operator-poc-rs # Kubernetes operator poc using rust
├── poe-trade-overlay    # trade overlay helper for poe
├── swkotor-mod          # attempt at modding kotor1
└── rojekti              # tmux session manager
```

## Prerequisites

- Darwin or Linux
- Nix

No other methods are supported nor ever will be. If you want to build or package
using different tools then fork and maintain them yourself. No patches for
fixing issues in other build or operating systems will be accepted.

## Developing

To build a project:

```bash
nix build '.#dir-name'

# Example fallout save editor
nix build '.#fallout-save-editor'

# Build artifacts created under ./result
```

To get a shell with all required dependencies:

```bash
nix develop '.#dir-name'

# Example fallout-save-editor
nix develop '.#fallout-save-editor'
```

To run a project:

```bash
nix run '.#dir-name'

# Example fallout-save-editor
nix run '.#fallout-save-editor'
```
