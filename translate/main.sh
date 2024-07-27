#!/usr/bin/env bash

# Defaults and optionals
default_translate_key="T"
translate_key="@tmux-translate-key"
default_window_width="40%"
window_width="@tmux-translate-width"
default_window_height="30%"
window_height="@tmux-translate-height"
default_src_lang="zh-cn"
src_lang="@tmux-translate-source-language"
default_dst_lang="en"
dst_lang="@tmux-translate-destination-language"
default_engine="Google"
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
    to_translate="$(tmux show-buffer)"
#	echo "Buffer content: $to_translate" > /tmp/tmux-translate-debug.log
    
    #translated_json="$(translate --from "$src_lang" --engine "$engine" --to "$dst_lang" --text "$to_translate")"
    translated_json="$(translate --from zh-cn --engine google --to en --text "$to_translate")"
    
   # echo "Translated JSON: $translated_json" >> /tmp/tmux-translate-debug.log

    translated_text="$(echo $translated_json | jq -r '.trans')"
    
    #echo "Translated text: $translated_text" >> /tmp/tmux-translate-debug.log
	tmux display-popup -w "$width" -h "$height" "echo 'Translated text: \n $translated_text'" 
}

translate_selection
