NAME=tetris-server
VERSION=$(shell git rev-parse HEAD)
TAG=aelred/$(NAME):$(VERSION)

build-server:
	DOCKER_BUILDKIT=1 docker build -t $(TAG) .

publish-server:
	docker push $(TAG)