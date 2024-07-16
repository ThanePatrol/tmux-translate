#!/usr/bin/env bash

CURRENT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
source "$CURRENT_DIR/core/main.sh"

set_key_bindings() {
	key="$(get_tmux_option "$translate_key" "$default_translate_key")"
	tmux bind-key -T copy-mode "$key" send-keys -X copy-pipe-and-cancel "$CURRENT_DIR/core/main.sh"
	tmux bind-key -T copy-mode-vi "$key" send-keys -X copy-pipe-and-cancel  "$CURRENT_DIR/core/main.sh"

	#tmux bind-key "$key" run-shell "$CURRENT_DIR/core/main.sh"
}

set_key_bindings

