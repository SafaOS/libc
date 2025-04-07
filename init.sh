#!/bin/bash
# run this script once every api commit bump
git submodule init
git submodule update --depth 1
cd api
./build.sh
cd ..
rm -rf link_with
cp -r api/out link_with