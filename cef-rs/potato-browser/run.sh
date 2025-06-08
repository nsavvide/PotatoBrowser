#!/bin/bash
export CEF_PATH="$HOME/.local/share/cef"
export LD_LIBRARY_PATH="$CEF_PATH/Release:$LD_LIBRARY_PATH"
cargo run

