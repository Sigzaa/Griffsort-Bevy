.DEFAULT_GOAL := dyn

dyn:
	cargo run --features "bevy/dynamic" client

release:
	cargo run --release

trace:
	cargo run --features "bevy/dynamic, bevy/tracy-trace" client

server:
	cargo run --features "bevy/dynamic" server

server-trace:
	cargo run --features "bevy/dynamic, bevy/tracy-trace" server
	
non-dyn:
	cargo run c