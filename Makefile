BIN_DIR = target/debug
INCLUDE_DIR = include
SRC_DIR = src

CC ?= gcc

CFLAGS += $(shell pkg-config --cflags sdl3) -I$(INCLUDE_DIR)
LDFLAGS += $(shell pkg-config --libs sdl3) -L$(BIN_DIR) -lled_strip_display

all: $(BIN_DIR)/sdl3-c-display

$(BIN_DIR)/sdl3-c-display: $(SRC_DIR)/bin/sdl3-c-display.c $(BIN_DIR)/libled_strip_display.a
	$(CC) $^ -o $@ $(CFLAGS) $(LDFLAGS)
