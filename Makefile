styles:
	sass --scss -t compressed --sourcemap=auto --unix-newlines static/scss/hawk.scss > static/dist/css/hawk.min.css

watch:
	sass --scss -t compressed --watch static/scss/hawk.scss:static/dist/css/hawk.min.css

build:
	cargo build --release

run:
	cargo run --release

dev:
	cargo run
