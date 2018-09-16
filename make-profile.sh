#!/usr/bin/env sh

errcho () {
    echo 1>&2 $@
}

if [ $# -eq 0 ]
then
    errcho "Expected binary name to profile"
    exit 1
fi

if [ $# -lt 2 ] || [ "$2" = "debug" ]
then
    FLAGS=
    FOLDER=debug
elif [ "$2" != "release" ]
then
    errcho "Invalid mode $2; Expecting either release or debug (default)"
    exit 1
else
    FLAGS=--release
    FOLDER=release
    export RUSTFLAGS='-g '"$RUSTFLAGS"
fi


pushd profiling > /dev/null || exit 1
cargo build $FLAGS --bin "$1" || exit 1
popd > /dev/null || exit 1

valgrind --tool=callgrind --demangle=yes ./profiling/target/$FOLDER/"$1" \
    || exit 1
