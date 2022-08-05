
PREFIX 	:= riscv32-unknown-elf
CC  	:= $(PREFIX)-gcc
AS  	:= $(PREFIX)-as
CFLAGS 	:= -march=rv32i -mabi=ilp32 -nostdlib
AFLAGS  := -march=rv32i
OBJDUMP := $(PREFIX)-objdump
OBJCOPY := $(PREFIX)-objcopy

all:
	$(CC) $(CFLAGS) -O0 test.c -o test.elf
	#$(AS) $(AFLAGS) test.s -o test.elf
	$(OBJCOPY) -O binary -j .text test.elf test.bin
disas:
	$(OBJDUMP) -M numeric,no-aliases -d test.elf 
clean:
	rm -fv *.{o,elf,bin}
