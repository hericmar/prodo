.DEFAULT_GOAL := build
export INSTALL_DIR := /usr/share/webapps/prodo
export VERSION := "$(shell git describe --tags --abbrev=0)"

build:
	@echo "Building prodo..."
	echo '{ "version": $(VERSION), "revHash": "$(shell git rev-parse --short HEAD)" }' > webapp/src/assets/version.json
	cd webapp && pnpm install && pnpm run build --mode pwa
	scripts/update_manifest.sh
	cp -r webapp/dist/pwa/* static/

install:
	@echo "Installing prodo..."
	scripts/install.sh

uninstall:
	@echo "Uninstalling prodo..."
	rm -rf /usr/share/webapps/prodo
	rm /usr/bin/prodo

