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

install -d -m 755 /usr/share/sddm/themes/breeze

if [ -f /usr/share/veyra/branding/sddm/theme.conf ]; then
    install -m 644         /usr/share/veyra/branding/sddm/theme.conf         /usr/share/sddm/themes/breeze/theme.conf
fi

install -d -m 755 /etc/sudoers.d

printf '%s\n' '%wheel ALL=(ALL:ALL) NOPASSWD: ALL' \
    > /etc/sudoers.d/10-veyra-live

chmod 440 /etc/sudoers.d/10-veyra-live

install -d -o veyra -g veyra -m 700 /home/veyra

# Restore executable permissions for Veyra commands after package installation.
find /usr/bin -maxdepth 1 -type f -name 'veyra*' -exec chmod 755 {} +
if [ -f /usr/bin/vpm ]; then
    chmod 755 /usr/bin/vpm
fi

exit 0
