#!/bin/bash

set -u

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PROFILE="$ROOT/veyra-iso"

PASS=0
FAIL=0

ok() {
    echo "✓ $1"
    PASS=$((PASS + 1))
}

bad() {
    echo "✗ $1"
    FAIL=$((FAIL + 1))
}

check() {
    if "$@"; then
        ok "$2"
    else
        bad "$2"
    fi
}

cd "$PROFILE" || exit 1

echo "========================================"
echo "       VEYRA FINAL PREFLIGHT"
echo "========================================"
echo

echo "=== Scripts ==="

for f in \
    veyra \
    veyra-center \
    veyra-menu \
    veyra-firstboot \
    veyra-settings \
    veyra-welcome \
    veyra-doctor \
    veyra-hardware \
    veyra-audio \
    veyra-print \
    veyra-power \
    veyra-firewall \
    veyra-installer \
    veyra-info \
    veyra-update \
    veyra-disks \
    veyra-about
do
    if bash -n "airootfs/usr/bin/$f" 2>/dev/null &&
       test -x "airootfs/usr/bin/$f"
    then
        ok "$f"
    else
        bad "$f"
    fi
done

echo
echo "=== Fastfetch ==="

if python -m json.tool \
    airootfs/etc/fastfetch/config.jsonc \
    >/dev/null 2>&1
then
    ok "Fastfetch JSON"
else
    bad "Fastfetch JSON"
fi

if grep -q '"type": "data-raw"' \
    airootfs/etc/fastfetch/config.jsonc
then
    ok "Text Veyra logo"
else
    bad "Fastfetch text logo"
fi

echo
echo "=== Network ==="

test -L \
    airootfs/etc/systemd/system/multi-user.target.wants/NetworkManager.service \
    && ok "NetworkManager enabled" \
    || bad "NetworkManager enabled"

test -L \
    airootfs/etc/systemd/system/multi-user.target.wants/systemd-resolved.service \
    && ok "systemd-resolved enabled" \
    || bad "systemd-resolved enabled"

test ! -e \
    airootfs/etc/systemd/system/sockets.target.wants/systemd-networkd.socket \
    && ok "systemd-networkd socket removed" \
    || bad "systemd-networkd socket still present"

test ! -e \
    airootfs/etc/systemd/system/dbus-org.freedesktop.network1.service \
    && ok "network1 alias removed" \
    || bad "network1 alias still present"

echo
echo "=== Time ==="

test -L \
    airootfs/etc/systemd/system/multi-user.target.wants/chronyd.service \
    && ok "chronyd enabled" \
    || bad "chronyd enabled"

test ! -e \
    airootfs/etc/systemd/system/sysinit.target.wants/systemd-timesyncd.service \
    && ok "timesyncd disabled" \
    || bad "timesyncd still enabled"

echo
echo "=== Core services ==="

for s in \
    bluetooth.service \
    cups.service \
    firewalld.service \
    power-profiles-daemon.service
do
    test -L \
        "airootfs/etc/systemd/system/multi-user.target.wants/$s" \
        && ok "$s" \
        || bad "$s"
done

echo
echo "=== Required configuration ==="

for f in \
    airootfs/etc/os-release \
    airootfs/etc/motd \
    airootfs/etc/fastfetch/config.jsonc \
    airootfs/etc/veyra/system.conf \
    airootfs/etc/veyra/branding.conf \
    airootfs/etc/veyra/repository.conf \
    airootfs/etc/NetworkManager/conf.d/10-veyra-dns.conf \
    airootfs/etc/sddm.conf.d/10-veyra.conf \
    airootfs/etc/skel/.config/kdeglobals \
    airootfs/etc/skel/.config/kwinrc \
    airootfs/etc/skel/.config/plasma-org.kde.plasma.desktop-appletsrc \
    airootfs/etc/skel/.config/autostart/veyra-firstboot.desktop
do
    test -e "$f" && ok "$f" || bad "$f"
done

echo
echo "=== Forbidden build artifacts ==="

if find . \
    -type f \
    \( \
        -name '*.iso' -o \
        -name '*.vdi' -o \
        -name '*.img' -o \
        -name '*.raw' \
    \) \
    -not -path './work/*' \
    -not -path './out/*' |
    grep -q .
then
    bad "ISO/VM artifacts inside profile"
else
    ok "No ISO/VM artifacts in profile"
fi

echo
echo "=== Package list ==="

PACKAGE_COUNT="$(
    awk '
        /^[[:space:]]*#/ { next }
        /^[[:space:]]*$/ { next }
        { count++ }
        END { print count+0 }
    ' packages.x86_64
)"

echo "Real packages: $PACKAGE_COUNT"

echo
echo "========================================"
echo "PASS: $PASS"
echo "FAIL: $FAIL"
echo "========================================"

if [ "$FAIL" -eq 0 ]; then
    echo
    echo "✓ VEYRA PREFLIGHT PASSED"
    exit 0
fi

echo
echo "✗ VEYRA PREFLIGHT FAILED"
exit 1
