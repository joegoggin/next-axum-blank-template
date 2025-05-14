server *args:
	cd server && cargo-watch -c -x "run -- {{args}}"

server-build:
	cd server && cargo-watch -c -x "build"
