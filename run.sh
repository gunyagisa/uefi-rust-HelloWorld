#!/bin/sh

mkdir -p fs/EFI/BOOT
cp target/x86_64-unknown-uefi/debug/Kos.efi fs/EFI/BOOT/BOOTX64.EFI

qemu-system-x86_64 -bios /usr/share/ovmf/OVMF.fd -hda fat:rw:fs
  
