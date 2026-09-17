# Veyra Architecture

## Overview

Veyra Linux is built as a complete system with a dedicated Veyra layer covering system management, desktop integration, package management, installation, and diagnostics.

## Desktop

The desktop layer provides:

- KDE Plasma integration
- Veyra Center
- Veyra Settings
- Veyra Welcome
- Veyra First Boot
- Veyra Fastfetch

## System Tools

The system layer provides:

- hardware diagnostics
- storage tools
- audio tools
- printing tools
- power management
- firewall management
- system diagnostics
- update tools

## Package Management

`vpm` provides repository synchronization, package searching, installation, removal, verification, and updates.

## Installation

`veyra-installer` provides guided and advanced installation flows.

## Repository

The Veyra package ecosystem is designed around:

    Veyra Repository
          |
       Veyra API
          |
         vpm
          |
      Veyra system

## Project Structure

    Veyra/
    ├── veyra-cli/
    ├── veyra-iso/
    ├── veyra-hello-pkg/
    ├── scripts/
    ├── docs/
    └── veyra-repo/

## Branding

Veyra uses its official visual identity across the system.

Project phrase:

> Made in Abduali
