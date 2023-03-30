#!/bin/bash
tmux -L fnstack -f ./tmux.conf new-session -c ./url-shortener -d "cargo run"
#tmux -L fnstack new-window -c ./url-shortener "cargo run"
tmux -L fnstack a
