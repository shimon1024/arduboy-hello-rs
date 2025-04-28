.PHONY: all setup cargo build upload clean

-include options.mk

ifndef PORT
  $(error PORT is not defined)
else
  $(shell printf 'PORT = %s\n' $(PORT) > options.mk)
endif

BOARD = arduino:avr:leonardo
RECIPE = "$$(arduino-cli compile -b $(BOARD) --show-properties=unexpanded \
		| grep -E '^recipe\.c\.combine\.pattern=.*$$' \
		| sed -r 's@(.*)@\1 target/avr-none/release/libhello.a@')"

all: build

setup:
	arduino-cli core install arduino:avr
	arduino-cli lib install Arduboy

cargo:
	cargo build --release

build: cargo
	arduino-cli compile --fqbn $(BOARD) --build-property $(RECIPE)

upload:
	arduino-cli upload --verify --fqbn $(BOARD) --port $(PORT)

clean:
	cargo clean

cleanall: clean
	rm -f options.mk
