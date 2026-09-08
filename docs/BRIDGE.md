# The C bridge

`omarchy_theme.h` is a separate contract from the Rust API `API.md`
promotes — it is *not* an extension of v1. v1 is palettes only: a basic
palette in, an extended legible one out. The C bridge exists because a v1
palette is a snapshot, and a real application needs to follow the active
theme as it changes and see it through more than one representation — both
of which sit behind types v1 explicitly keeps out of its own surface
(`ThemeWatcher`, `Representation`). The bridge is where those two live, in a
form any language with C FFI can call.

## Ownership

Four rules cover every pointer that crosses the bridge, so no function needs to
be reread to confirm the pattern:

| What | Who owns it | What the caller does |
| --- | --- | --- |
| The strings from `omarchy_role_name`, `omarchy_color_name`, `omarchy_representation_name` | The library | Reads them; never frees them. They are static and outlive any watcher. An out-of-range index gives `NULL`. |
| The `OmarchyWatcher *` from `omarchy_watch` | The caller | Frees it exactly once, with `omarchy_free`. Never twice, never while another call on it is running. |
| The `OmarchySnapshot` passed to `omarchy_current` and `omarchy_poll_changed` | The caller | Provides the storage; the library fills it by copy. It holds no pointers, so it stays valid after the watcher is freed. |
| The `error_buffer` passed to `omarchy_watch` | The caller | Provides it and its size; the library writes a null-terminated message into it only on failure. Passing `NULL` is allowed. |

The file descriptor from `omarchy_signal_fd` belongs to the watcher: read from
it, never close it, and stop using it once the watcher is freed.

## Usage flow

```c
#include "omarchy_theme.h"
#include <stdio.h>
#include <poll.h>
#include <unistd.h>

int main(void) {
    char error[256];
    OmarchyWatcher *watcher = omarchy_watch(NULL, error, sizeof(error));
    if (watcher == NULL) {
        fprintf(stderr, "could not watch the theme: %s\n", error);
        return 1;
    }

    OmarchySnapshot snapshot;
    omarchy_current(watcher, 0, &snapshot); /* 0 = Original */
    printf("starting on %s theme\n", snapshot.dark ? "a dark" : "a light");

    struct pollfd watched = { .fd = omarchy_signal_fd(watcher), .events = POLLIN };
    while (poll(&watched, 1, -1) > 0) {
        if (omarchy_poll_changed(watcher, 0, &snapshot)) {
            printf("theme changed — now %s\n", snapshot.dark ? "dark" : "light");
        }
    }

    omarchy_free(watcher);
    return 0;
}
```

`omarchy_signal_fd` gives a file descriptor readable whenever a valid change
landed — integrate it into whatever event loop the application already runs
(`poll`, `select`, Qt's `QSocketNotifier`, a Go `epoll` wrapper, …) instead of
polling on a timer. Reading it never blocks; draining it fully before the
next wait is the caller's job, same as any other readiness fd.

### In a Qt event loop

The same flow with no `poll()` of its own, because the application already has
an event loop. This is what the reference application in this repository does —
see `gui/bridge/ThemeBridge.cpp`.

```cpp
ThemeBridge::ThemeBridge(QObject *parent) : QObject(parent) {
    std::array<char, 256> error{};
    m_watcher = omarchy_watch(nullptr, error.data(), error.size());
    if (m_watcher == nullptr) {
        m_startupError = QString::fromUtf8(error.data());
        return;
    }

    refresh();  // draw once on what the theme is right now

    m_notifier = new QSocketNotifier(omarchy_signal_fd(m_watcher),
                                     QSocketNotifier::Read, this);
    connect(m_notifier, &QSocketNotifier::activated,
            this, &ThemeBridge::onSignalReadable);
}

ThemeBridge::~ThemeBridge() {
    omarchy_free(m_watcher);
}

void ThemeBridge::onSignalReadable() {
    // Drain the fd before polling: several changes can coalesce between two
    // wakeups, and only the newest one still matters.
    std::array<char, 64> discard{};
    const auto fd = static_cast<int>(m_notifier->socket());
    while (read(fd, discard.data(), discard.size()) > 0) {
    }

    OmarchySnapshot snapshot{};
    if (omarchy_poll_changed(m_watcher, m_representation, &snapshot)) {
        applySnapshot(snapshot);  // redraw
    }
}
```

The shape is the same in any event loop: register the fd, drain it when it
wakes, ask the watcher whether anything actually changed, redraw only if it
did.

`omarchy_current` and `omarchy_poll_changed` both take a representation index
— see below — and write a full `OmarchySnapshot` either way; the difference
is that `omarchy_poll_changed` returns `false` and leaves `*out` untouched
when nothing changed since the last call.

## Thread safety

`omarchy_current` and `omarchy_poll_changed` may be called from any thread
for as long as the `OmarchyWatcher` they take is alive. The state behind a
watcher sits behind a mutex; the thread that watches the filesystem for
changes updates it independently of whatever thread is reading. A read never
blocks waiting on that thread, and never returns a torn snapshot.

`omarchy_free` is the one call that isn't safe to race: never call it
concurrently with another call on the same watcher, and never call it twice
on the same pointer.

## Representations

Every function that takes a `size_t representation` reads it against this
fixed order — the same `omarchy_representation_name` names:

| Index | Name | What it does |
| --- | --- | --- |
| 0 | Original | The theme exactly as declared. |
| 1 | Inverted | The same identity with its polarity flipped — dark becomes light or light becomes dark, and each color keeps its own hue, only restated for the ground it now sits on. |
| 2 | HighContrast | Every color pushed further from the background than it already stood, for readability. |
| 3 | Mono | The theme's own scale, background to foreground, with no hue. |
| 4 | Print | Forced onto a white page: pure white behind, pure black text, every other color reduced to the grey its own weight gives it. |

An out-of-range index falls back to Original rather than failing — there is
no error path for it.

## Versioning

The shared object's SONAME (`libomarchy_theme.so.1`) names the ABI
generation: the layout of `OmarchyColor`, `OmarchyRole`, `OmarchyShade`,
`OmarchyColorFamily`, `OmarchySnapshot`, and the signature of every
`omarchy_*` function. Past `1.0.0` it moves whenever one of those breaks.

Before `1.0.0` it does not. The package version (`Cargo.toml`, the `.pc`
file, the CMake package) is `1.0.0-beta.N` right now, and a beta is the
generation still being shaped: a layout may change from one beta to the next
— a role added to `OmarchySnapshot`, a field placed differently — without the
SONAME moving off `.1`. Recompile against the header of the beta you link.

The package version is what a consumer pins during the beta, and every such
change advances it. The bridge earns `1.0.0` without a suffix once an
application outside this repository has tried it; from there the SONAME
carries the promise, and a break moves it to `.2`.
