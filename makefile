CC = rustc

test_src = $(wildcard test/*.rs)

TEST = $(test_src:.rs=)
LIB = liblist.rlib

all: $(LIB)

$(LIB): list.rs
	$(CC) --crate-name list --crate-type lib $< -o $@

$(TEST): %: %.rs $(LIB)
	$(CC) $< --test --extern list=$(LIB) -o $@

test: test/main
	./$< --test $(CASE)

clean:
	rm -f $(LIB) $(TEST)

.PHONY: test clean
