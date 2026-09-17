#!/bin/bash
set -euo pipefail

if ! id veyra >/dev/null 2>&1; then
    useradd \
        --create-home \
        --uid 1000 \
        --shell /bin/bash \
        --groups wheel,audio,video,storage,power \
        veyra
fi

passwd -d veyra

install -d -m 755 /etc/sudoers.d

printf '%s\n' '%wheel ALL=(ALL:ALL) NOPASSWD: ALL' \
    > /etc/sudoers.d/10-veyra-live

chmod 440 /etc/sudoers.d/10-veyra-live

install -d -o veyra -g veyra -m 700 /home/veyra

exit 0
