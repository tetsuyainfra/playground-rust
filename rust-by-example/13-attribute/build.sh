#!/bin/bash

set -ex

# crate.rs内でアトリビュートを指定しているので
# --crate-typeを指定しなくてもコンパイルできる
rustc crate.rs

ls *rary*