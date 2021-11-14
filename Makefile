all:
	cargo build --release
install: 
	install -D -m 755 target/release/cola2 ${HOME}/bin
uninstall:
	rm -v ${HOME}/bin/cola2
