#!/bin/bash

# Remove Zig Source
rm -rf src
rm -rf .zigcache
rm build.zig
rm build.zig.zon

# Init Zig
zig init
