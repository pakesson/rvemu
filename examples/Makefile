FILES = example1.bin

%.o: %.s
	riscv64-linux-gnu-as -march=rv64i -o $@ $<

%.bin: %.o
	riscv64-linux-gnu-objcopy -O binary $< $@

all: $(FILES)

clean:
	rm -f *.o
	rm -f *.bin
