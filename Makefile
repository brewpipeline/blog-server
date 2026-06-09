BLOG_UI_TAG ?= 1.4.2

blog-ui:
	git clone --depth 1 --branch $(BLOG_UI_TAG) https://github.com/brewpipeline/blog-ui.git blog-ui

run: blog-ui
	cargo run -p blog-server-api

build: blog-ui
	cargo build -p blog-server-api

.PHONY: run build
