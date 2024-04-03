########################################################################

# Проверки разные по коду и форматирование
# Так можно делать таргеты закрытыми с помощью аттрибутов:
# - https://just.systems/man/en/chapter_32.html
[private]
@_FMT_CHECK_CLIPPY:
    cargo fmt \
        --all
    cargo check \
        --all-targets
    cargo clippy \
        --all-targets

[private]
@_FMT_CHECK_CLIPPY_RELEASE:
    cargo fmt \
        --all
    cargo check \
        --release \
        --all-targets
    cargo clippy \
        --release \
        --all-targets

########################################################################

BUILD: _FMT_CHECK_CLIPPY
    cargo build

BUILD_RELEASE: _FMT_CHECK_CLIPPY_RELEASE
    cargo build \
        --release

########################################################################

RUN: BUILD
    {{justfile_directory()}}/target/debug/test_1