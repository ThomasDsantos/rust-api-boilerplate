# Simple Makefile for a Go project

all: compose

compose:
	@docker compose --profile="*" down --remove-orphans && \
	docker compose --profile="*" up --build --watch

.PHONY: all compose


