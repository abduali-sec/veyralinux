# Veyra Development

## Source Tree

    Veyra/
    ├── veyra-cli/
    ├── veyra-iso/
    ├── veyra-hello-pkg/
    ├── scripts/
    ├── docs/
    └── veyra-repo/

## Shell Validation

    bash -n veyra-iso/airootfs/usr/bin/veyra

Validate all Veyra shell programs:

    for f in veyra-iso/airootfs/usr/bin/veyra*; do
        bash -n "$f" || exit 1
    done

## Rust

    cd veyra-cli
    cargo check
    cargo build --release

## Preflight

    ./scripts/final-preflight.sh

## Git

    git status
    GIT_PAGER=cat git diff
