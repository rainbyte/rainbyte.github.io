.PHONY: build rebuild watch

SITE_CMD = cargo run
SITE_DEST = docs/
SITE_PORT = 9000

build:
	$(SITE_CMD)

rebuild:
	rm -rf $(SITE_DEST)
	$(SITE_CMD)

watch:
	python -m http.server -d $(SITE_DEST) $(SITE_PORT)

push: rebuild
	@echo -e '\nPushing blog...\n'
	git add docs
	git commit -m "Site update"
	git push
	@echo -e '\nSite updated...\n'
