#!/usr/bin/env bash

CURRENT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
main="$CURRENT_DIR/main.sh"

# shellcheck source=main.sh
source "$main"

set_key_bindings() {
	key="$(get_tmux_option "$translate_key_option" "$default_translate_key")"
	tmux bind-key -T copy-mode "$key" send-keys -X copy-pipe-and-cancel "$main"
	tmux bind-key -T copy-mode-vi "$key" send-keys -X copy-pipe-and-cancel  "$main"
}

set_key_bindings

