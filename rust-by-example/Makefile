SOURCES=$(wildcard ./*.rs)
USELESS=$(patsubst %.rs,%,$(SOURCES))

all: $(USELESS)

$(USELESS): $(SOURCES)
	rustc $@.rs

clean:
	rm -rf $(USELESS)
