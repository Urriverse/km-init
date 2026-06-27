PROFILE ?= debug
KM_INIT_PROFILE ?= $(PROFILE)
BUILD_PATH ?= target
RUSTFLAGS += "-C relocation-model=pie -C code-model=kernel -C no-redzone=y"

build:
	@CARGO_TARGET_DIR="$(BUILD_PATH)" cargo build $(KM_INIT_CARGO_ARGS) --target x86_64-unknown-none
	@if [ ! -z "$(ARTEFACT_PATH)" ]; then echo "$(BUILD_PATH)/km-init/x86_64-unknown-none/$(KM_INIT_PROFILE)/km-init" > $(ARTEFACT_PATH); fi

test:
	@cargo test
