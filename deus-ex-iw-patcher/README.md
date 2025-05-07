# Patcher for Deus Ex Invisible War

Enables patching Deus Ex Invisible War game files

# Features

* Patch games hardcoded 85 FOV to better match your resolution

Not everything is automated. But manual instructions are printed by commands
when required.

Only tested on Steam version and on Linux (proton).

# Usage

```bash
deus-ex-iw-patcher patch-resolution \
    "$HOME/.local/share/Steam/steamapps/common/Deus Ex Invisible War/System/Engine.d2u" \
    -x 2560  \
    -y 1440
```

Which should then print further instructions like:

```
File patched, please set the following manually:

In 'steamapps/common/Deus Ex Invisible War/System/Default.ini':
  FOV__d=53
  AssumedUIScreenWidth__d=1440

In 'pfx/drive_c/users/steamuser/Documents/Deus Ex - Invisible War/user.ini':
  FullscreenViewportY=1440
  FullscreenViewportX=2560
```

This will create a file called `Engine.d2u.new` in the backup old `Engine.d2u`
and replace with this.

# Credits

I did none of the reverse-engineering, I just monkied half-assed automation
based on this post: https://www.wsgf.org/dr/deus-ex-invisible-war
