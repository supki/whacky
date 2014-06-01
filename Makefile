RUSTC ?= rustc
RUSTCFLASGS ?= --opt-level=3 --crate-type=bin

BUILDDIR ?= build
TARGET := $(BUILDDIR)/whacky

MKDIR := mkdir --parents
RM := rm
RMDIR := rmdir --ignore-fail-on-non-empty
STRIP := strip --strip-all

all: $(BUILDDIR)/whacky

$(TARGET): src/main.rs
	$(MKDIR) $(BUILDDIR)
	$(RUSTC) $(RUSTCFLASGS) -o $@ $<
	$(STRIP) $@

clean:
	$(RM) $(TARGET)
	$(RMDIR) $(BUILDDIR)

.PHONY: all clean
