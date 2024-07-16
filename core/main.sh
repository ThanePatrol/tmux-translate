#!/usr/bin/env bash

# TODO - some common format for string format of source, dst langs + engine
# Defaults and optionals
default_translate_key="T"
translate_key="@tmux-translate-key"
default_window_width="40%"
window_width="@tmux-translate-width"
default_window_height="30%"
window_height="@tmux-translate-height"
default_src_lang="cn"
src_lang="@tmux-translate-source-language"
default_dst_lang="en"
dst_lang="@tmux-translate-destination-language"
default_engine="google"
engine="@tmux-translate-engine"

get_tmux_option() {
	local option value default
	option="$1"
	default="$2"
	value=$(tmux show-option -gqv "$option")

	if [ -n "$value" ]; then
		echo "$value"
	else
		echo "$default"
	fi
}

translate_selection() {
	width="$(get_tmux_option $window_width $default_window_width)"
	height="$(get_tmux_option $window_height $default_window_height)"
	tmux display-popup -w "$width" -h "$height" "echo 'translated output here buffer is $(tmux show-buffer)'" 
}

translate_selection
