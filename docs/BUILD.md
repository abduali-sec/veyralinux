# Building Veyra Linux

## Build Requirements

The build machine requires:

- archiso
- mkinitcpio
- mkinitcpio-archiso
- Rust
- Cargo

Verify the tools:

    pacman -Q archiso mkinitcpio mkinitcpio-archiso

    cargo --version

## Build Veyra Package Manager

From the project root:

    ./scripts/build-vpm.sh

## Build the ISO

    cd ~/Veyra/veyra-iso
    sudo rm -rf work out
    mkdir -p work out
    sudo mkarchiso -v -w work -o out .

## Verify the ISO

    ls -lh out/*.iso
    sha256sum out/*.iso

Generated ISO and build directories should not be committed to Git.
