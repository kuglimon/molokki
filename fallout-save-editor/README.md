# Fallout Save Editor

Save game editor for Fallout 2.

Features:

* Some what documented parser for saves
* Can fix NCR aggro in save files
* Nix based build, everything just works
* Tested, ~15k lines of test just for map parsing

# But why?

Sulik didn't put his weapon away and now half of NCR want's to kill me. I
stunlocked myself on making my own editor and introduce some forced positivity
into NCR.

Some of other available mod tools were flagged by totalvirus as viruses. Others
were closed source, which I think doesn't help the modding community at all.

# Usage

```bash
# Make all NCR cops in downtown friendly again
fallout-save-editor --save-file-path ./NCR1.SAV fix-ncr-cop-aggro
```

# Compiling

```bash
nix build
```

# Developing

```bash
# If you use bash
nix develop

# Something else
nix develop -c zsh
```
