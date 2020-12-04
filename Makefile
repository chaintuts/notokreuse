# This file contains a make script for the NotOKReuse application
#
# Author: Josh McIntyre
#

# This block defines makefile variables
BUILD_DIR=target

# This rule builds the application
# Here we are simply wrapping Rust's cargo tool
build:
	cargo build

# This rule cleans the build directory
clean: $(BUILD_DIR)
	rm -r $(BUILD_DIR)/*
	rmdir $(BUILD_DIR)
