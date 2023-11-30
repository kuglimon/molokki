# Rojekti

Tmuxinator inspired tmux layout manager.

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

There's hacky packaging for Arch Linux:

```bash
cd pkg && makepkg -si
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
