TARGET    := aarch64-unknown-none
PROJ_NAME := uart_01
OBJ_NAME  := target/$(TARGET)/release/$(PROJ_NAME)
OBJCOPY   := rust-objcopy --binary-architecture=aarch64

all: clean kernel8.img

target/$(TARGET)/release/$(PROJ_NAME): src/main.rs
	RUST_TARGET_PATH=$(shell pwd) cargo build --target=$(TARGET) --release

kernel8: target/$(TARGET)/release/$(PROJ_NAME)

kernel8.img: kernel8
	$(OBJCOPY) $(OBJ_NAME) --strip-all -O binary $@

clean:
	cargo clean
	rm -f target
	rm -f kernel8.img

