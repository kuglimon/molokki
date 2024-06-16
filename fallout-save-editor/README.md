# Fallout Save Editor

Save game editor for Fallout 2.

Features:

* Some what documented parser for saves
* Can fix NCR aggro in save files
* Nix based build, everything just works
* Tested, ~15k lines of test just for map parsing

# But why?

Sulik didn't put his weapon away and now half of NCR wants to kill me. I
stunlocked myself to make my own save editor and introduce some forced pacifism
into NCR.

Some of the other available mod tools were flagged as viruses by totalvirus.
Many were closed source, which doesn't help the modding community at all.

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
