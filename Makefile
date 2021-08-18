CARGO := cargo

OBJS := $(wildcard src/*.rs)

.PHONY: all clean

all: target/release/poke-parser

target/release/poke-parser: $(OBJS)
	@$(CARGO) build --release

clean:
	$(RM) target
