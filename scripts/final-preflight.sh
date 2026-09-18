#!/bin/bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ISO="$ROOT/veyra-iso"

ok() {
    echo "✓ $1"
}

fail() {
    echo "✗ $1"
    exit 1
}

cd "$ROOT"

echo "== BUILD TOOLS =="

command -v mkarchiso >/dev/null || fail "mkarchiso missing"
command -v mkinitcpio >/dev/null || fail "mkinitcpio missing"
command -v cargo >/dev/null || fail "cargo missing"

pacman -Q archiso >/dev/null 2>&1 || fail "archiso package missing"
pacman -Q mkinitcpio >/dev/null 2>&1 || fail "mkinitcpio package missing"
pacman -Q mkinitcpio-archiso >/dev/null 2>&1 || fail "mkinitcpio-archiso package missing"

test -f /usr/lib/initcpio/hooks/archiso || fail "archiso hook missing"
test -f /usr/lib/initcpio/install/archiso || fail "archiso install hook missing"

ok "build tools and archiso hooks"

echo
echo "== LIVE DESKTOP =="

CUSTOMIZE="$ISO/airootfs/root/customize_airootfs.sh"

test -f "$CUSTOMIZE"     || fail "customize_airootfs.sh missing"

test -x "$CUSTOMIZE"     || fail "customize_airootfs.sh is not executable"

bash -n "$CUSTOMIZE"     || fail "customize_airootfs.sh syntax error"

grep -q 'useradd' "$CUSTOMIZE"     || fail "live user creation missing"

grep -q 'veyra' "$CUSTOMIZE"     || fail "veyra user missing from customize script"

test -f "$ISO/airootfs/etc/sddm.conf.d/10-veyra.conf"     || fail "SDDM config missing"

grep -q '^User=veyra$'     "$ISO/airootfs/etc/sddm.conf.d/10-veyra.conf"     || fail "SDDM autologin user missing"

grep -q '^Session=plasma.desktop$'     "$ISO/airootfs/etc/sddm.conf.d/10-veyra.conf"     || fail "Plasma session missing"

test -f "$ISO/airootfs/etc/hostname"     || fail "hostname missing"

grep -qxF 'veyra' "$ISO/airootfs/etc/hostname"     || fail "hostname is not veyra"

if find "$ISO/airootfs/etc/systemd/system"     -name '*veyra-live-user*'     -print | grep -q .; then
    fail "old live-user service still exists"
fi

ok "Live user build setup and SDDM"

echo
echo "== NETWORK/SERVICE CLEANUP =="

if find "$ISO/airootfs/etc/systemd/system" \
    \( -name 'cloud-config.service' \
    -o -name 'cloud-final.service' \
    -o -name 'cloud-init-local.service' \
    -o -name 'cloud-init-main.service' \
    -o -name 'cloud-init-network.service' \
    -o -name 'systemd-timesyncd.service' \
    -o -name 'dbus-org.freedesktop.network1.service' \
    -o -name 'dbus-org.freedesktop.timesync1.service' \) \
    -print | grep -q .; then
    fail "stale service links remain"
fi

ok "stale services removed"

echo
echo "== FASTFETCH =="

test -f "$ISO/airootfs/etc/fastfetch/config.jsonc" \
    || fail "Fastfetch config missing"

python - "$ISO/airootfs/etc/fastfetch/config.jsonc" <<'PY'
import json
import sys

with open(sys.argv[1], "r", encoding="utf-8") as f:
    json.load(f)
PY

grep -q '"type": "data-raw"' \
    "$ISO/airootfs/etc/fastfetch/config.jsonc" \
    || fail "Fastfetch is not using text logo"

if find "$ISO/airootfs/usr/share/veyra/branding/fastfetch" \
    -type f 2>/dev/null | grep -q .; then
    fail "Fastfetch branding contains image files"
fi

ok "Fastfetch"

echo
echo "== VPM =="

test -f "$ISO/airootfs/usr/bin/vpm" || fail "vpm missing"
test -x "$ISO/airootfs/usr/bin/vpm" || fail "vpm not executable"

ok "vpm"

echo
echo "== ISO PROFILE =="

test -f "$ISO/profiledef.sh" || fail "profiledef.sh missing"
test -f "$ISO/packages.x86_64" || fail "packages.x86_64 missing"
grep -qxF 'mkinitcpio-archiso' "$ISO/packages.x86_64" \
    || fail "mkinitcpio-archiso missing from package list"

ok "ISO profile"

echo
echo "== Veyra executable permissions =="
for f in     veyra     veyra-about     veyra-audio     veyra-center     veyra-disks     veyra-doctor     veyra-firewall     veyra-firstboot     veyra-hardware     veyra-info     veyra-installer     veyra-menu     veyra-power     veyra-print     veyra-settings     veyra-update     veyra-welcome
do
    test -x "$ROOT/veyra-iso/airootfs/usr/bin/$f"         || fail "Veyra command is not executable: $f"
done

if [ -e "$ROOT/veyra-iso/airootfs/usr/bin/vpm" ]; then
    test -x "$ROOT/veyra-iso/airootfs/usr/bin/vpm"         || fail "vpm is not executable"
fi

ok "Veyra executable permissions"

echo "== BUILD ARTIFACTS =="

ARTIFACTS=$(find "$ROOT" -maxdepth 3 \
    \( -name '*.iso' -o -name '*.vdi' -o -name '*.img' -o -name '*.raw' \) \
    -not -path "$ISO/out/*" \
    -not -path "$ISO/work/*" \
    -not -path "$ROOT/build-work" \
    -not -path "$ROOT/build-work/*" \
    -not -path "$ROOT/veyra-test.vdi" \
    -not -path "$ROOT/veyra-install-test-0.1.0.vdi" \
    -print)

if [[ -n "$ARTIFACTS" ]]; then
    echo "$ARTIFACTS"
    fail "unexpected build artifact found"
fi

ok "artifact check"

echo
echo "========================================"
echo "VEYRA FINAL PREFLIGHT PASSED"
echo "========================================"
