TARGET := skernel_aarch64
TARGET_ELF := ../../target/aarch64-unknown-none/release/$(TARGET)

all: $(TARGET) FORCE

$(TARGET): $(TARGET_ELF)
	@rust-objcopy --binary-architecture=aarch64 --strip-all -O binary $< $@

$(TARGET_ELF):
	@cargo build -p $(TARGET) --target aarch64-unknown-none --release

clean:
	@rm -rf ./$(TARGET)
	@cargo clean -p $(TARGET) --target aarch64-unknown-none --release
FORCE:

.PHONY: FORCE

