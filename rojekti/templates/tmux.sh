#!/usr/bin/env bash

tmux start-server

cd {{ root }} %>

# Run project on start hooks
# TODO(tatu): I've yet to use this

if tmux has-session -t "{{ name }}" &>/dev/null; then
  # TODO(tatu): Needs window indexing
  echo "new session"
  {% for window in windows -%}
    tmux new-window "path" -t #{tmux_window_target} -n {{ window.name }}
  {% endfor -%}
else
  echo "existing session"
  # TODO(tatu): Implement existing session support
fi

{% if attach  -%}
if [ -z "$TMUX" ]; then
  tmux -u attach-session -t {{ name }}
else
  tmux -u switch-client -t {{ name }}
fi
{% endif -%}

