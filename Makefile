$(shell touch .env)

SQLX = cargo sqlx
DOCKER = docker

include .env
export

setup-network:
	${DOCKER} network remove app
	${DOCKER} network create app --driver bridge

deploy-db:
	${DOCKER} run -d --name postgres \
		--network app \
		-e POSTGRES_PASSWORD=password \
		-p 5432:5432 \
		postgres:latest

deploy:
	${SQLX} prepare
	${DOCKER} build --no-cache -t web-service:latest-local .
	${DOCKER} rm -f web-service || true
	${DOCKER} run -d --name web-service \
		--network app \
		-p 3000:3000 \
		web-service:latest-local