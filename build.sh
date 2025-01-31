cargo build --target x86_64-unknown-uefi --release
dd if=/dev/zero of=boot.img bs=1M count=64
mkfs.fat ./boot.img
rm -rf esp
mkdir esp
mount boot.img esp
cd esp && mkdir -p ./EFI/BOOT
cd ..
cp target/x86_64-unknown-uefi/release/uefi_boot.efi ./esp/efi/boot/bootx64.efi
cp bzImage ./esp/efi/boot/bzImage.efi
umount esp
qemu-img convert -f raw -O vhdx ./boot.img boot.vhdx