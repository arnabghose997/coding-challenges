BINARY_NAME:=lb

.PHONY: build-windows build-unix

build-windows:
	cargo build
	if not exist artifact mkdir artifact
	copy .\target\debug\$(BINARY_NAME).exe .\artifact\$(BINARY_NAME).exe

build-unix:
	cargo build
	mkdir -p artifact
	cp target/debug/$(BINARY_NAME) artifact/$(BINARY_NAME)
