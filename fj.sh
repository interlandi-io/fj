#!/bin/zsh

PUSHD_PREFIX='__pushd__'

fj () {
    if [ $# -eq 0 ]; then
        popd
    else
        local result=$($HOME/repo/fj/target/debug/fj "$1")
        if [[ $result == "$PUSHD_PREFIX"* ]]; then
            pushd "${result#__pushd__}"
        else 
            echo $result
        fi
    fi
}
