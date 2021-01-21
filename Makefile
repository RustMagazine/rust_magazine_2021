.PHONY: deploy

init:
	git worktree add /tmp/book gh-pages

deploy: book
	@echo "====> deploying to github"
	mdbook build
	rm -rf /tmp/book/*
	cp -rp book/* /tmp/book/
	cd /tmp/book && \
		git add -A && \
		git commit -m "deployed on $(shell date) by ${USER}" && \
		git push origin gh-pages