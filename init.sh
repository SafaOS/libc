#!/bin/bash
# run this script once every api commit bump
git submodule init
git submodule update --depth 1
cd api
./build.sh
cd ..
mkdir -p out
cp -r api/out/* out