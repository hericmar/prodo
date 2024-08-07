.DEFAULT_GOAL := build
export INSTALL_DIR := /usr/share/webapps/prodo

build:
	@echo "Building prodo..."
	scripts/bump_version.sh
	cargo build --release
	cd webapp && pnpm install && pnpm run build --mode pwa && cd ..

install:
	@echo "Installing prodo..."
	scripts/install.sh

uninstall:
	@echo "Uninstalling prodo..."
	rm -rf /usr/share/webapps/prodo
	rm /usr/bin/prodo

