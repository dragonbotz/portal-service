#!/bin/bash

cargo build --release
if [[ $? -gt 0 ]]; then
	return $?
fi

sudo docker build \
	-f service.Dockerfile \
	-t dbz-portal-service \
	--no-cache \
	.

sudo docker volume create dbz-portal-database-volume
