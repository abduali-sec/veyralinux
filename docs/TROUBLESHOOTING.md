# Troubleshooting

## ISO Does Not Boot

Verify the ISO checksum:

    sha256sum <iso-file>

## Veyra Command Does Not Start

Check permissions:

    ls -l /usr/bin/veyra

The program must have executable permissions.

## Veyra Package Manager Cannot Synchronize

Check network connectivity:

    curl -I https://veyra-api.abdualialderson.workers.dev/

Then retry:

    vpm sync

## Installer Problems

Run:

    veyra doctor

Then verify the target disk, partition layout, network connection, available storage, and boot mode.

## Graphics Problems

Check the current session:

    echo "$XDG_SESSION_TYPE"

Check display configuration:

    kscreen-doctor -o

## Hardware Problems

Run:

    veyra hardware

## General Diagnostics

Run:

    veyra doctor
