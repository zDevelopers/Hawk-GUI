styles:
	sass -s compressed static/scss/hawk.scss:static/dist/css/hawk.min.css

watch:
	sass -s compressed --watch static/scss/hawk.scss:static/dist/css/hawk.min.css

install-dev:
	pipenv install --dev
	maturin develop

link-dev:
	maturin develop

install-release:
	pipenv install
	maturin develop --release # TODO update
