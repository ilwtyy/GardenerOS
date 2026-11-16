# Building
TARGET := riscv64gc-unknown-none-elf
MODE := release
KERNEL_ELF := target/$(TARGET)/$(MODE)/os
KERNEL_BIN := $(KERNEL_ELF).bin

# Bootloader
BOOTLOADER := ../bootloader/opensbi.bin

# Kernel entry
KERNEL_ENTRY_PA := 0x80200000

# Binutils
OBJDUMP := rust-objdump --arch-name=riscv64
OBJCOPY := rust-objcopy --binary-architecture=riscv64

.PHONY: build kernel run clean disasm

# Compile the kernel
kernel:
	@cargo build --release

# Generate binary image
build: kernel
	@$(OBJCOPY) $(KERNEL_ELF) --strip-all -O binary $(KERNEL_BIN)

# Run the kernel with QEMU
run: build
	@qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios $(BOOTLOADER) \
		-device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA)

# Disassemble the kernel
disasm: kernel
	@$(OBJDUMP) -d $(KERNEL_ELF)

# Clean build files
clean:
	@cargo clean
