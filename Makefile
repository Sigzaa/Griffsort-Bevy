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

	mkdir -p package/$(VER)/windows package/$(VER)/linux package/$(VER)/compressed package/$(VER)/windows/scripts package/$(VER)/linux/scripts

	cp target/x86_64-pc-windows-gnu/release/Griffsort.exe package/$(VER)/windows
	cp -r ./assets package/$(VER)/windows
	cp -r ./config package/$(VER)/windows
	cp -r ./package-scripts/windows/* package/$(VER)/windows/scripts/
	cd package/$(VER)/windows; zip -r ../compressed/griffsort-$(VER)-win.zip ./

	cp target/release/Griffsort package/$(VER)/linux
	cp -r ./assets package/$(VER)/linux
	cp -r ./config package/$(VER)/linux
	cp -r ./package-scripts/linux/* package/$(VER)/linux/scripts/
	cd package/$(VER)/linux; zip -r ../compressed/griffsort-$(VER)-linux.zip ./

	gh release create $(VER) package/$(VER)/compressed/*

draft:
	cargo build --release
	cargo build --target=x86_64-pc-windows-gnu --release

	mkdir -p package/$(VER)/windows package/$(VER)/linux package/$(VER)/compressed package/$(VER)/windows/scripts package/$(VER)/linux/scripts

	cp target/release/Griffsort package/$(VER)/linux
	cp -r ./package-scripts/linux/* package/$(VER)/linux/scripts/
	cp -r ./assets package/$(VER)/linux
	cp -r ./config package/$(VER)/linux

	cp target/x86_64-pc-windows-gnu/release/Griffsort.exe package/$(VER)/windows
	cp -r ./package-scripts/windows/* package/$(VER)/windows/scripts/
	cp -r ./assets package/$(VER)/windows
	cp -r ./config package/$(VER)/windows


clear: