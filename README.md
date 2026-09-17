# Veyra Linux

**Veyra Linux** is a Linux system built around the Veyra ecosystem.

Veyra brings together the desktop environment, system tools, package management, installation, diagnostics, and other components into one unified system.

> **Made in Abduali**

## Current Release

**Veyra Linux 0.1.0 — Genesis**

## Main Components

- `veyra` — main Veyra command-line interface
- `veyra-center` — unified Veyra control center
- `veyra-settings` — Veyra settings interface
- `veyra-installer` — Veyra system installer
- `veyra-doctor` — system diagnostics
- `veyra-hardware` — hardware information and diagnostics
- `veyra-audio` — audio diagnostics and controls
- `veyra-print` — printing and scanning tools
- `veyra-power` — power profile management
- `veyra-firewall` — firewall management
- `veyra-disks` — disk management tools
- `veyra-update` — Veyra update system
- `veyra-info` — Veyra system information
- `vpm` — Veyra Package Manager

## Veyra Package Manager

`vpm` is the package manager used by Veyra Linux.

Synchronize repository metadata:

    vpm sync

Search for a package:

    vpm search firefox

Install a package:

    sudo vpm install <package>

Remove a package:

    sudo vpm remove <package>

Update Veyra packages:

    sudo vpm update

Verify a package:

    vpm verify <package>

## Desktop

Veyra integrates its own tools and services into the desktop environment, including:

- Veyra Center
- Veyra Settings
- Veyra Welcome
- Veyra First Boot
- Veyra Doctor
- Veyra Fastfetch
- system management tools
- hardware diagnostics
- networking and service management

## Installation

Veyra includes its own installer:

    veyra installer

The installer provides guided and advanced installation modes for supported BIOS and UEFI systems.

## Repository

Veyra Repository:

https://github.com/abduali-sec/veyra-repo

## API

Veyra API:

https://veyra-api.abdualialderson.workers.dev/

## Support

https://t.me/Veyralinuxx

## Project

https://github.com/abduali-sec/veyralinux

## Documentation

- [Installation](docs/INSTALL.md)
- [Veyra Package Manager](docs/VPM.md)
- [Veyra CLI](docs/CLI.md)
- [Architecture](docs/ARCHITECTURE.md)
- [Building Veyra](docs/BUILD.md)
- [Development](docs/DEVELOPMENT.md)
- [Troubleshooting](docs/TROUBLESHOOTING.md)
- [FAQ](docs/FAQ.md)
- [Release Process](docs/RELEASE.md)

## Project Status

Veyra Linux 0.1.0 is the Genesis release and is currently undergoing final testing and release preparation.

## License

License information will be added before the first public release.

---

**Veyra Linux — Made in Abduali**
