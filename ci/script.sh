# This script takes care of testing your crate

set -ex

# TODO This is the "test phase", tweak it as you see fit
main() {
    cross build --target $TARGET --all
    cross build --target $TARGET --release --all

    if [ ! -z $DISABLE_TESTS ]; then
        return
    fi

    cross test --target $TARGET --all
    cross test --target $TARGET --release --all

    cross run --target $TARGET --all
    cross run --target $TARGET --release --all
}

# we don't run the "test phase" when doing deploys
if [ -z $TRAVIS_TAG ]; then
    main
fi
