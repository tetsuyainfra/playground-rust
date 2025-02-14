#!/usr/bin/env bash
set -ex

git remote -v

MYURL=$(git config --get remote.origin.url)
REPO=$(echo $MYURL | rev | cut -d '/' -f 1,2 | rev)
echo $REPO

git remote set-url origin git@github.com:${REPO} https://github.com/${REPO}

git remote -v