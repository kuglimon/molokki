# Rojekti

Tmuxinator inspired tmux layout manager.

Code is worth a thousand pictures:

```yaml
name: rojekti
root: ~/development/personal/molokki/rojekti

pre_window: nix develop -c zsh

windows:
  - editor: vim
  - sandbox: null
  - release: null
```

This gives me three panes in nix development shell with all my dependencies and
an editor to get started on coding rojekti.

## Differences with Tmuxinator

* ships with one binary with everything statically linked
* simple installation, no gems/ruby/runtime to manage
* starting new sessions is around 50% faster
* not all commands exist or work the same way
* templates subjectively easier to debug

Ruby startup time is not awesome. While it's not the end of the world it does
feel annoying. Just the `list` the command help takes closer to 100ms on a beefy
desktop.

Shell script templates have debugging support and generally try to have less
variation. More checks are done on the scripts rather than executing shell
commands from Ruby before template generation.

## Requirements

* bash
* tmux
* rust (build only)

## Compiling

Use cargo:

```bash
cargo build --release
```

## Installing

Include as flake:

```nix
rojekti = {
  url = "github:UncertainSchrodinger/molokki?dir=rojekti";
  inputs.nixpkgs.follows = "nixpkgs";
};

# And then in some host configuration
packages = with pkgs; [ rojekti ]
```

### macOS

```bash
brew tap uncertainschrodinger/tap
brew install rojekti --head
```

## Debugging

Check what kind of bash template it creates and check it for errors:

```bash
rojekti debug project_name
```

If it looks okay then try running with debugging enabled:

```bash
DEBUG=1 rojekti start project_name
```

## Tips

### Faster startup

Tmux start up isn't free even on a rather fast machine. It's best to start it
during boot from init/systemd and configure tmux not to shutdown if there are no
sessions with:

```
# set this in tmux.conf
set-option -g exit-empty off
```

## Naming

Dialect of a Finnish word for project. I just didn't spend a lot of time
thinking about the name...

## License

Parts are copied from [tmuxinator](https://github.com/tmuxinator/tmuxinator) and
thus are likely licensed based on its license.

Otherwise sources follow the repository license.
