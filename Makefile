.DEFAULT_GOAL := build
export INSTALL_DIR := /usr/share/webapps/prodo

build:
	@echo "Building prodo..."
	cd webapp && pnpm install && pnpm run build --mode pwa
	cp -r webapp/dist/pwa/* static/

install:
	@echo "Installing prodo..."
	scripts/install.sh

uninstall:
	@echo "Uninstalling prodo..."
	rm -rf /usr/share/webapps/prodo
	rm /usr/bin/prodo

