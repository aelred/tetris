NAME=tetris-server
VERSION=$(shell git rev-parse HEAD)
TAG=aelred/$(NAME):$(VERSION)

docker-login:
	docker login

build-server:
	DOCKER_BUILDKIT=1 docker build -t $(TAG) .

publish-server: docker-login
	docker push $(TAG)