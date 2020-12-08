styles:
	sass --style compressed static/scss/hawk.scss:static/dist/css/hawk.min.css

watch:
	sass --style compressed --watch static/scss/hawk.scss:static/dist/css/hawk.min.css

run-back:
	python manage.py runserver

install-dev:
	pipenv install --dev
	maturin develop

link-dev:
	maturin develop

install-release:
	pipenv install
	maturin develop --release # TODO update

run: link-dev
	make -j2 run-back watch
