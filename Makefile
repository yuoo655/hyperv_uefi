qemu:
	qemu-system-x86_64 \
	-m 1G \
  	-drive if=pflash,format=raw,readonly=on,file=OVMF_CODE.fd \
  	-drive if=pflash,format=raw,readonly=on,file=OVMF_VARS.fd \
	-drive format=raw,file=fat:rw:esp \
	-net none \
  	-drive format=raw,file=boot.img \
	--nographic