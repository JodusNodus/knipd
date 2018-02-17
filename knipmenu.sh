run () {
  item=$(knip -l | kies -p "copy:")
	if [ -n "$item" ]; then
		printf "$item" | ./target/release/knip -p | pbcopy
	fi
}
