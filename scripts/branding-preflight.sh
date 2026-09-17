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

echo "== OFFICIAL ASSETS =="

for f in \
  "$ROOT/assets/branding/logo/veyra-logo-icon.png" \
  "$ROOT/assets/branding/logo/veyra-logo-square.png" \
  "$ROOT/assets/branding/reference/veyra-icon-sheet.jpeg" \
  "$ROOT/assets/branding/reference/veyra-logo-variants-reference.jpeg"
do
    test -f "$f" || fail "missing asset: $f"
done

ok "official project assets"

echo
echo "== ISO BRANDING =="

for f in \
  "$ISO/airootfs/usr/share/veyra/branding/logo/veyra-logo-icon.png" \
  "$ISO/airootfs/usr/share/veyra/branding/logo/veyra-logo-square.png" \
  "$ISO/airootfs/usr/share/veyra/branding/icons/veyra.png" \
  "$ISO/airootfs/usr/share/veyra/branding/sddm/veyra-logo.png"
do
    test -f "$f" || fail "missing ISO branding asset: $f"
done

ok "canonical ISO branding assets"

echo
echo "== APPLICATION ICONS =="

for f in "$ISO"/airootfs/usr/share/applications/veyra-*.desktop; do
    test -f "$f" || continue
    grep -q '^Icon=veyra$' "$f" || fail "wrong icon: $(basename "$f")"
done

ok "Veyra application icons"

echo
echo "== SDDM =="

SDDM="$ISO/airootfs/usr/share/sddm/themes/breeze/theme.conf"

test -f "$SDDM" || fail "SDDM theme configuration missing"

grep -q '^showlogo=shown$' "$SDDM" \
    || fail "Veyra logo is not enabled in SDDM"

grep -q '^logo=/usr/share/veyra/branding/sddm/veyra-logo.png$' "$SDDM" \
    || fail "SDDM is not using canonical Veyra logo"

ok "SDDM branding"

echo
echo "== FASTFETCH =="

FF="$ISO/airootfs/etc/fastfetch/config.jsonc"

test -f "$FF" || fail "Fastfetch config missing"

grep -q '"type": "data-raw"' "$FF" \
    || fail "Fastfetch no longer uses the text logo"

ok "Fastfetch text branding preserved"

echo
echo "== REFERENCE FILES =="

if find "$ISO/airootfs/usr/share/veyra/branding" \
    -type f \( -name '*.jpeg' -o -name '*.jpg' \) \
    -print | grep -q .; then
    fail "reference JPEG files were copied into ISO branding"
fi

ok "reference-only images are not in ISO"

echo
echo "== OLD USER-FACING BRANDING =="

if grep -RniE \
  'Arch Linux Live|Arch Linux|archlinux' \
  "$ISO/airootfs/etc/os-release" \
  "$ISO/airootfs/etc/motd" \
  "$ISO/airootfs/etc/sddm.conf.d" \
  "$ISO/airootfs/usr/share/applications" \
  2>/dev/null; then
    fail "old user-facing branding found"
fi

ok "no old user-facing branding"

echo
echo "========================================"
echo "VEYRA BRANDING PREFLIGHT PASSED"
echo "========================================"
