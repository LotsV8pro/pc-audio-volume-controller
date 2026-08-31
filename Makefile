PREFIX ?= /usr/local
BINDIR = $(DESTDIR)$(PREFIX)/bin
APPDIR = $(DESTDIR)$(PREFIX)/share/applications
ICONDIR = $(DESTDIR)$(PREFIX)/share/icons/hicolor/scalable/apps

APP = lotus-audio
DESKTOP = $(APP).desktop
ICON = icons/hicolor/scalable/apps/$(APP).svg

.PHONY: all build install uninstall clean run

all: build

build:
	cargo build --release

install: build
	install -Dm755 target/release/$(APP) $(BINDIR)/$(APP)
	install -Dm644 $(DESKTOP) $(APPDIR)/$(DESKTOP)
	install -Dm644 $(ICON) $(ICONDIR)/$(APP).svg
	@command -v gtk-update-icon-cache >/dev/null 2>&1 && gtk-update-icon-cache -f -t $(DESTDIR)$(PREFIX)/share/icons/hicolor >/dev/null 2>&1 || true
	@command -v update-desktop-database >/dev/null 2>&1 && update-desktop-database $(APPDIR) >/dev/null 2>&1 || true

uninstall:
	rm -f $(BINDIR)/$(APP)
	rm -f $(APPDIR)/$(DESKTOP)
	rm -f $(ICONDIR)/$(APP).svg

clean:
	cargo clean

run: build
	./target/release/$(APP)
