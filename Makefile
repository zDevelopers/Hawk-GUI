styles:
	sass -s compressed static/scss/hawk.scss:static/dist/css/hawk.min.css

watch:
	sass -s compressed --watch static/scss/hawk.scss:static/dist/css/hawk.min.css

install-dev:
	pipenv install --dev
	pyo3-pack develop

link-dev:
	pyo3-pack develop

install-release:
	pipenv install
	pyo3-pack develop --release
