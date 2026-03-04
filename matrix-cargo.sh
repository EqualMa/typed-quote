#!/bin/bash

set -eu

if [[ "${1:0:1}" == "+" ]]; then
    my_command="$2"
else
    my_command="$1"
fi

my_extra_args=()
if [[ "$my_command" == "clippy" ]]; then
    my_extra_args=("--all-targets" "--" "-W" "clippy::cargo" "-D" "warnings")
fi

cargo "$@" --no-default-features                        "${my_extra_args[@]+"${my_extra_args[@]}"}"
cargo "$@" --no-default-features --features proc-macro  "${my_extra_args[@]+"${my_extra_args[@]}"}"
cargo "$@" --no-default-features --features proc-macro2 "${my_extra_args[@]+"${my_extra_args[@]}"}"
cargo "$@"                                              "${my_extra_args[@]+"${my_extra_args[@]}"}"
cargo "$@" --features proc-macro                        "${my_extra_args[@]+"${my_extra_args[@]}"}"
cargo "$@" --features proc-macro2                       "${my_extra_args[@]+"${my_extra_args[@]}"}"
cargo "$@" --all-features                               "${my_extra_args[@]+"${my_extra_args[@]}"}"
