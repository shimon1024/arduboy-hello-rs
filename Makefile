.PHONY: all setup cargo build upload

-include options.mk

ifndef PORT
  $(error PORT is not defined)
else
  $(shell printf 'PORT = %s\n' $(PORT) > options.mk)
endif

BOARD = arduino:avr:leonardo
TARGET = avr-none-atmega32u4
RECIPE = "$$(arduino-cli compile -b $(BOARD) --show-properties \
		| grep -E '^recipe\.c\.combine\.pattern=.*$$' \
		| sed -r 's@(.*)@\1 target/$(TARGET)/release/libhello.a@')"

all: build

setup:
	rustc --print target-spec-json -Z unstable-options \
		--target avr-none -C target-cpu=atmega32u4 \
		> $(TARGET).json
	arduino-cli core install arduino:avr
	arduino-cli lib install Arduboy

cargo:
	cargo build --target $(TARGET).json --release

build: cargo
	arduino-cli compile --fqbn $(BOARD) --build-property $(RECIPE)

upload:
	arduino-cli upload --verify --fqbn $(BOARD) --port $(PORT)
