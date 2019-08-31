styles:
	sass -s compressed static/scss/hawk.scss:static/dist/css/hawk.min.css

watch:
	sass -s compressed --watch static/scss/hawk.scss:static/dist/css/hawk.min.css

build:
	cargo build --release

run:
	cargo run --release

dev:
	cargo run

link-dev:
	cargo build
	mv target/debug/libhawk_processing.so target/debug/hawk_processing.so
