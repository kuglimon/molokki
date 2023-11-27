# Rojekti

Tmuxinator but with rust

## Differences

* ships with one binary with everything statically linked
* simple installation and no gems/ruby/runtime to manage
* faster, Starting new sessions is around 50% faster
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

## Naming

Dialect of a Finnish word for project. I just didn't spend a lot of time
thinking about the name...

## License

Parts are copied from [tmuxinator](https://github.com/tmuxinator/tmuxinator) and
thus are likely licensed based on its license.

Otherwise sources follow the repository license.
