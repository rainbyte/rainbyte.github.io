.PHONY: build rebuild watch

SITE_CMD = stack exec -- site

build:
	$(SITE_CMD) $@

rebuild:
	$(SITE_CMD) $@

watch:
	$(SITE_CMD) $@ --host 0.0.0.0

push: rebuild
	@echo -e '\nPushing blog...\n'
	git add docs
	git commit -m "Site update"
	git push
	@echo -e '\nSite updated...\n'
