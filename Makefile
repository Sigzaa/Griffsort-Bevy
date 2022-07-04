.DEFAULT_GOAL := dyn

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

publish || p:
	cargo build --release
	cargo build --target=x86_64-pc-windows-gnu --release
	
	rm -R package
	mkdir -p package/linux package/windows

	cp target/release/Griffsort package/linux
	cp -r ./assets package/linux
	zip -r package/griffsort-lin.zip package/linux

	cp target/x86_64-pc-windows-gnu/release/Griffsort.exe package/windows
	cp -r ./assets package/windows
	zip -r package/griffsort-win.zip package/windows
