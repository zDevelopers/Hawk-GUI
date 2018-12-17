styles:
	sass --scss -t compressed --sourcemap=auto --unix-newlines static/scss/uhcreloaded-reports.scss > static/dist/css/uhcreloaded-reports.min.css

watch:
	sass --scss -t compressed --watch static/scss/uhcreloaded-reports.scss:static/dist/css/uhcreloaded-reports.min.css

build:
	cargo build --release

run:
	cargo run --release

dev:
	cargo run
