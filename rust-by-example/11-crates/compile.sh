#!/bin/bash

set -ex

rustc --crate-type=lib rary.rs
ls lib*
echo ---

# 外部クレートが参照できなくてErrorになる
# rustc executable.rs
rustc executable.rs --extern rary=library.rlib
./executable

