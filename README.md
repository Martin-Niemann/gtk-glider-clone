An experiment in trying to recreate the Hacker News Android reader app Glider (version 1.18) for Linux smartphones using a Linux-native GUI toolkit.

### Currently implemented

* Fetch stories and show them as cards in a list.

### Building inside an ARM-based Debian Sid container running in Distrobox

Building and running the app from inside a Debian Sid container in Distrobox currently works on my OnePlus 6 running postmarketOS, though gestures in the app are broken.

#### Debian dependencies

`build-essential pkg-config libglib2.0-dev libpango1.0-dev libgraphene-1.0-dev libgtk-4-dev libadwaita-1-dev`

### Building on ARM postmarketOS edge

The app currently builds on my Oneplus 6, but segfaults when run. This is when builing with and without `export RUSTFLAGS="-C target-feature=-crt-static"`.

#### Dependencies

Some of these, like bash completions, may be unnecessary.

`alpine-sdk openssl openssl-dev libadwaita libadwaita-dev gtk4.0 gtk4.0-dev pango pango-dev graphene graphene-dev gdk-pixbuf gdk-pixbuf-dev glib glib-dev rust cargo cargo-bash-completions cairo cairo-dev`
