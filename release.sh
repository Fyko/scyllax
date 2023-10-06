#!/bin/sh 

cargo release --workspace --exclude book --exclude example $@
