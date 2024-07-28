#!/usr/bin/env bash

# Defaults and optionals
default_translate_key="t"
translate_key_option="@tmux-translate-key"
default_window_width="40%"
window_width_option="@tmux-translate-width"
default_window_height="30%"
window_height_option="@tmux-translate-height"
default_src_lang="zh-cn"
src_lang_option="@tmux-translate-source-language"
default_dst_lang="en"
dst_lang_option="@tmux-translate-destination-language"
default_engine="google"
engine_option="@tmux-translate-engine"

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
	width="$(get_tmux_option $window_width_option $default_window_width)"
	height="$(get_tmux_option $window_height_option $default_window_height)"
	src_lang="$(get_tmux_option $src_lang_option $default_src_lang)"
	dst_lang="$(get_tmux_option $dst_lang_option $default_dst_lang)"
	engine="$(get_tmux_option $engine_option $default_engine)"

    to_translate="$(tmux show-buffer)"
    
    translated_json="$(translate --from "$src_lang" --engine "$engine" --to "$dst_lang" --text "$to_translate")"
    
	translated_text="$(format_output "$translated_json")"
    
	tmux display-popup -w "$width" -h "$height" "echo 'Translated text: \n $translated_text'" 
}

format_output() {
	json_input="$1"
	output_str="$(echo "$json_input" | jq -r '.trans')\n"

	# get 3 closest translation terms from dictionary if they exist and join with with commas, else nothing
	output_str="$output_str $(echo "$json_input" | jq -r 'if .dict[0].terms then .dict[0].terms[1:4] | join(", ") else "" end')"
	echo "$output_str"
}

translate_selection
