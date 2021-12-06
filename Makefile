.PHONY: deploy

init:
	git worktree add -f /tmp/book gh-pages
	git worktree remove -f /tmp/book
	git worktree add -f /tmp/book gh-pages

deploy: init
	@echo "====> deploying to github"
	mdbook build
	rm -rf /tmp/book/*
	cp -rp book/* /tmp/book/
	cd /tmp/book && \
		git add -A && \
		git commit -m "deployed on $(shell date) by ${USER}" && \
		git push -f origin gh-pages