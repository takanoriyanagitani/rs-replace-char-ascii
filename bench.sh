#!/bin/sh

randsource=/dev/urandom

input=./sample.d/rand.dat
count=16384

gendata(){
	echo generating data...

	mkdir -p sample.d

	dd \
		if="${randsource}" \
		of="${input}" \
		bs=1048576 \
		count=$count \
		status=progress
}

bench_native(){
	cat "${input}" |
		\time \
			-l \
			env \
				ENV_REPLACE_BEFORE="	" \
				ENV_REPLACE_AFTER=" " \
				./rs-replace-char-ascii |
		dd \
			if=/dev/stdin \
			of=/dev/zero \
			bs=1048576 \
			status=progress
}

bench_wazero(){
	cat "${input}" |
		\time -l \
		wazero \
			run \
			--env ENV_REPLACE_BEFORE="	" \
			--env ENV_REPLACE_AFTER=" " \
			./rs-replace-char-ascii.wasm |
		dd \
			if=/dev/stdin \
			of=/dev/zero \
			bs=1048576 \
			status=progress
}

test -f "${input}" || gendata

#bench_native
bench_wazero
