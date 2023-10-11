#!/bin/bash

debug_mode=false

# 解析命令行选项
while getopts ":d" opt; do
    case $opt in
        d)
            debug_mode=true
            ;;
        \?)
            echo "Invalid option: -$OPTARG" >&2
            exit 1
            ;;
    esac
done

# 移除已处理的选项
shift $((OPTIND - 1))

# 检查是否以 debug 模式运行
if $debug_mode; then
    echo "Running in debug mode"
    qemu-system-riscv64 \
    -machine virt \
    -nographic \
    -bios ../bootloader/rustsbi-qemu.bin \
    -device loader,file=target/riscv64gc-unknown-none-elf/release/os.bin,addr=0x80200000 \
    -s -S
else
    echo "Running in normal mode"
    qemu-system-riscv64 \
    -machine virt \
    -nographic \
    -bios ../bootloader/rustsbi-qemu.bin \
    -device loader,file=target/riscv64gc-unknown-none-elf/release/os.bin,addr=0x80200000
fi

# 处理其它参数
if [ $# -gt 0 ]; then
    echo "Additional arguments: $@"
    # 在这里处理其它参数
fi