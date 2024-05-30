#!/usr/bin/env bash

./docker-build.sh
docker run -it --rm -p 3000:3000 --name swissgoat cpollet/swissgoat