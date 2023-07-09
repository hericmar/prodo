.DEFAULT_GOAL := build
export INSTALL_DIR := /usr/share/webapps/prodo

build:
	@echo "Building prodo..."
	cd webapp
	echo '{ "version": "$(shell git describe --tags --abbrev=0)", "revHash": "$(shell git rev-parse --short HEAD)" }' > src/assets/version.json
	pnpm install && pnpm run build --mode pwa
	cp -r webapp/dist/pwa/* static/

install:
	@echo "Installing prodo..."
	scripts/install.sh

uninstall:
	@echo "Uninstalling prodo..."
	rm -rf /usr/share/webapps/prodo
	rm /usr/bin/prodo

