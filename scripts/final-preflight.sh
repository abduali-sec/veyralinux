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
echo "== VEYRA PROGRAMS =="

PROGRAMS=(
  veyra
  veyra-center
  veyra-menu
  veyra-firstboot
  veyra-settings
  veyra-welcome
  veyra-doctor
  veyra-hardware
  veyra-audio
  veyra-print
  veyra-power
  veyra-firewall
  veyra-installer
  veyra-info
  veyra-update
  veyra-disks
  veyra-about
)

for p in "${PROGRAMS[@]}"; do
    f="$ISO/airootfs/usr/bin/$p"

    test -f "$f" || fail "missing $p"
    test -x "$f" || fail "$p is not executable"

    case "$p" in
        veyra-firstboot|veyra-settings|veyra-welcome|veyra-doctor|veyra-hardware|veyra-audio|veyra-print|veyra-power|veyra-firewall|veyra-installer|veyra-info|veyra-update|veyra-disks|veyra-about|veyra|veyra-center|veyra-menu)
            bash -n "$f" || fail "syntax error: $p"
            ;;
    esac
done

test -f "$ISO/airootfs/usr/bin/vpm" || fail "vpm missing"
test -x "$ISO/airootfs/usr/bin/vpm" || fail "vpm is not executable"

ok "all Veyra programs"

echo
echo "== LIVE DESKTOP =="

test -f "$ISO/airootfs/usr/bin/veyra-live-user-setup" \
    || fail "live user setup missing"

test -x "$ISO/airootfs/usr/bin/veyra-live-user-setup" \
    || fail "live user setup not executable"

bash -n "$ISO/airootfs/usr/bin/veyra-live-user-setup" \
    || fail "live user setup syntax error"

test -f "$ISO/airootfs/etc/systemd/system/veyra-live-user.service" \
    || fail "live user service missing"

test -L "$ISO/airootfs/etc/systemd/system/multi-user.target.wants/veyra-live-user.service" \
    || fail "live user service is not enabled"

test -f "$ISO/airootfs/etc/sddm.conf.d/10-veyra.conf" \
    || fail "SDDM config missing"

grep -q '^User=veyra$' \
    "$ISO/airootfs/etc/sddm.conf.d/10-veyra.conf" \
    || fail "SDDM autologin user missing"

grep -q '^Session=plasma.desktop$' \
    "$ISO/airootfs/etc/sddm.conf.d/10-veyra.conf" \
    || fail "Plasma session missing"

test -f "$ISO/airootfs/etc/hostname" \
    || fail "hostname missing"

grep -qxF 'veyra' "$ISO/airootfs/etc/hostname" \
    || fail "hostname is not veyra"

ok "Live user and SDDM"

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
echo "== BUILD ARTIFACTS =="

ARTIFACTS=$(find "$ROOT" -maxdepth 3 \
    \( -name '*.iso' -o -name '*.vdi' -o -name '*.img' -o -name '*.raw' \) \
    -not -path "$ISO/out/*" \
    -not -path "$ISO/work/*" \
    -not -path "$ROOT/veyra-test.vdi" \
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
