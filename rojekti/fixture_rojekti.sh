#!/usr/bin/zsh

# Clear rbenv variables before starting tmux
unset RBENV_VERSION
unset RBENV_DIR

tmux start-server;

cd /home/kuglimon/development/personal/molokki.git/main/rojekti

# Run on_project_start command.



  # Run pre command.
  
  
  # Run on_project_first_start command.
  

  # Create the session and the first window. Manually switch to root
  # directory if required to support tmux < 1.9
  TMUX= tmux new-session -d -s rojekti -n editor
  tmux send-keys -t rojekti:1 cd\ /home/kuglimon/development/personal/molokki.git/main/rojekti C-m


  # Create other windows.
  tmux new-window -c /home/kuglimon/development/personal/molokki.git/main/rojekti -t rojekti:2 -n sandbox
  tmux new-window -c /home/kuglimon/development/personal/molokki.git/main/rojekti -t rojekti:3 -n release


  # Window "editor"
  tmux send-keys -t rojekti:1 vim C-m


  # Window "sandbox"


  # Window "release"


  tmux select-window -t rojekti:1
  tmux select-pane -t rojekti:1.1

  if [ -z "$TMUX" ]; then
    tmux -u attach-session -t rojekti
  else
    tmux -u switch-client -t rojekti
  fi


    
# Run on_project_exit command.

