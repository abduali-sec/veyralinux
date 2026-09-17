#!/usr/bin/env bash
# shellcheck disable=SC2034

iso_name="veyra"
iso_label="VEYRA_$(date --date="@${SOURCE_DATE_EPOCH:-$(date +%s)}" +%Y%m)"
iso_publisher="Veyra Linux — Made in Abduali"
iso_application="Veyra Linux Live"
iso_version="$(date --date="@${SOURCE_DATE_EPOCH:-$(date +%s)}" +%Y.%m.%d)"

install_dir="veyra"

buildmodes=('iso')

arch="x86_64"

pacman_conf="pacman.conf"

bootmodes=(
    'bios.syslinux'
    'uefi.systemd-boot'
)

airootfs_image_type="squashfs"

airootfs_image_tool_options=(
    '-comp' 'xz'
    '-Xbcj' 'x86,arm64'
    '-b' '1M'
    '-Xdict-size' '1M'
)
