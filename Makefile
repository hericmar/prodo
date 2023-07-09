.DEFAULT_GOAL := build
export INSTALL_DIR := /usr/share/webapps/prodo

build:
	@echo "Building prodo..."
	echo '{ "version": "$(shell git describe --tags --abbrev=0)", "revHash": "$(shell git rev-parse --short HEAD)" }' > webapp/src/assets/version.json
	cd webapp && pnpm install && pnpm run build --mode pwa
	jq '.version = "$(shell git describe --tags --abbrev=0)"' webapp/dist/pwa/manifest.json > webapp/dist/pwa/manifest.json
	cp -r webapp/dist/pwa/* static/

install:
	@echo "Installing prodo..."
	scripts/install.sh

uninstall:
	@echo "Uninstalling prodo..."
	rm -rf /usr/share/webapps/prodo
	rm /usr/bin/prodo

