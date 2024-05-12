#!/usr/bin/env bash

CURRENT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
tmux bind-key T run-shell "$CURRENT_DIR/core/list.sh"

get_tmux_option() {
	local option value default
	option="$1"
	default="$2"
	value=$(tmux show-option -gqv "$option")

	if [ -n "$value" ]; then
		if [ "$value" = "null" ]; then
			echo ""
		else 
			echo "$value"
		fi
	else
		echo "$default"
	fi
}
