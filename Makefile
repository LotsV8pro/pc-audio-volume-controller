PREFIX ?= /usr/local
BINDIR = $(DESTDIR)$(PREFIX)/bin
APPDIR = $(DESTDIR)$(PREFIX)/share/applications
ICONDIR = $(DESTDIR)$(PREFIX)/share/icons/hicolor/scalable/apps

APP = lotus-audio
DESKTOP = $(APP).desktop

.PHONY: all build install uninstall clean run

all: build

build:
	cargo build --release

install: build
	install -Dm755 target/release/$(APP) $(BINDIR)/$(APP)
	install -Dm644 $(DESKTOP) $(APPDIR)/$(DESKTOP)

uninstall:
	rm -f $(BINDIR)/$(APP)
	rm -f $(APPDIR)/$(DESKTOP)

clean:
	cargo clean

run: build
	./target/release/$(APP)
