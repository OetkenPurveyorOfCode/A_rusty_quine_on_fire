#!/bin/bash
set -xe
rustc stage1.rs -o stage1
rustc squarify.rs -o squarify
./squarify
rustc stage2.rs -o stage2
