.PHONY: test node_modules

all: node_modules build

node_modules:
	yarn install

test:
	npm run test

upgrade:
	yarn outdated | awk 'NR>2{print $$1}' | sed '$$d' | xargs yarn upgrade

lint:
	npm run lint

build:
	npm run webpack

build-watch:
	npm run webpack:watch

help:
	cat Makefile
