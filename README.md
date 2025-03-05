# Hyper-V UEFI Linux Boot

本项目旨在通过Hyper-V中用uefi方式启动linux

## 快速开始

```bash
wget https://cdn.kernel.org/pub/linux/kernel/v6.x/linux-6.13.tar.xz
tar -xf linux-6.13.tar.xz
cd linux-6.13
cp ../hypervconfig ./.config
make -j$(nproc)
cp ./arch/x86/boot/bzImage ../ && cd ..
sh build.sh
```
进入Hyper-v管理器->新建->第二代->连接虚拟硬盘->使用现有虚拟硬盘(boot.vhdx)

虚拟机创建完成之后在设置中把安全启动关闭,之后点击启动
