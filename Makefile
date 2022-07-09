.DEFAULT_GOAL := dyn

VER := $(shell eval cargo pkgid | sed 's/^.*Griffsort://')

dyn:
	cargo run --features "bevy/dynamic" client

release || r:
	cargo run --release client

tracy || t:
	cargo run --features "bevy/dynamic, bevy/trace_tracy" client

server || s:
	cargo run --features "bevy/dynamic" server
	

server-tracy || st:
	cargo run --features "bevy/dynamic, bevy/trace_tracy" server
	
classic || c:
	cargo run client
build || b:
	cargo build --release	

publish:

	cargo build --release
	cargo build --target=x86_64-pc-windows-gnu --release

	mkdir -p package/$(VER)/windows package/$(VER)/linux package/$(VER)/compressed 

	cp target/release/Griffsort package/$(VER)/linux
	cp -r ./assets package/$(VER)/linux
	cp -r ./config package/$(VER)/linux
	zip -r package/$(VER)/compressed/griffsort-$(VER)-linux.zip package/$(VER)/linux

	cp target/x86_64-pc-windows-gnu/release/Griffsort.exe package/$(VER)/windows
	cp -r ./assets package/$(VER)/windows
	cp -r ./config package/$(VER)/windows
	zip -r package/$(VER)/compressed/griffsort-$(VER)-win.zip package/$(VER)/windows

	gh release create $(VER) package/$(VER)/compressed/*

clear:
	rm -R package