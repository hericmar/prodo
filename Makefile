.DEFAULT_GOAL := build
export INSTALL_DIR := /usr/share/webapps/prodo

build:
	@echo "Building prodo..."
	cd webapp && pnpm install && pnpm run build
	cp -r webapp/dist/spa/* static/
