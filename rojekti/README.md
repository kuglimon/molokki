# Rojekti

Tmuxinator but with rust

## Differences

Made with rust. Ships with one binary with everything statically linked, simple
installation and no gems/ruby to manage.

Ruby startup time is not awesome. While it's not the end of the world it does
feel annoying. Just listing the command help takes closer to 100ms. There's
probably room for improvement as well. Starting new sessions is around 50%
faster but it's still too slow.

Certain commands don't exist and some are different. For example `start` is a
combination of `new` and `start` in `tmuxinator`.

Shell script templates it renders are more readable and easier to debug.

## Requirements

* bash
* tmux

## Naming

Dialect of a Finnish word for project. I just didn't spend a lot of time
thinking about the name...

