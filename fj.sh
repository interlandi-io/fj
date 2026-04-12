#!/bin/zsh

fj () {
    if [ $# -eq 0 ]; then
        popd
    else
        pushd $(eval "~/repo/fj/target/debug/fj '$1'")
    fi
}
