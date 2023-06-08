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

mkdir .tmp

cp res/init_database.sql .tmp
cp res/*.conf .tmp

cd .tmp
sudo docker build \
	-f ../database.Dockerfile \
	-t dbz-portal-service-db \
	.
cd ..

rm -r .tmp
