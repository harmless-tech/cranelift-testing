all:
	cargo run
	cd testing && make clean && make && ./main.x
	
x86_64-apple:
	cargo run -- x86_64-apple-darwin
	cd testing && make clean && make && ./main.x
	
arm64-linux:
	cargo run -- aarch64-unknown-linux-gnu
	cd testing && make clean && make && set -e && ./main.x
	
x86_64-linux:
	cargo run -- x86_64-unknown-linux-gnu
	cd testing && make clean && make && ./main.x

current_dir = $(shell pwd)
docker:
	docker run -it --rm --platform=linux/arm64 -e CARGO_TARGET_DIR=/target -v ${HOME}/.cargo/registry:/usr/local/cargo/registry -v $(current_dir):/ctest -w /ctest rust:latest bash

docker-amd64:
	docker run -it --rm --platform=linux/amd64 -e CARGO_TARGET_DIR=/target -v ${HOME}/.cargo/registry:/usr/local/cargo/registry -v $(current_dir):/ctest -w /ctest rust:latest bash
