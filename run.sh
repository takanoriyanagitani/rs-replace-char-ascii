#!/bin/sh

printf 'hello\tworld\n' |
	env \
		ENV_REPLACE_BEFORE='	' \
		ENV_REPLACE_AFTER=, \
		./rs-replace-char-ascii
