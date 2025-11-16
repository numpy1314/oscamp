TARGET := aarch64-unknown-none
MODE := release
OUT_DIR := ../../target/$(TARGET)/$(MODE)
OUT_BIN := $(OUT_DIR)/skernel_aarch64

.PHONY: all clean build

all: build

build:
	@cargo build --target $(TARGET) --release
	@rust-objcopy --binary-architecture=aarch64 $(OUT_BIN) --strip-all -O binary $(OUT_BIN).bin
	@echo "Built: $(OUT_BIN).bin"

clean:
	@cargo clean
