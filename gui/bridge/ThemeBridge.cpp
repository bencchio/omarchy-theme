#include "ThemeBridge.h"

#include <array>
#include <unistd.h>
#include <QColor>
#include <QVariantMap>

namespace {

constexpr size_t ERROR_BUFFER_SIZE = 256;

QColor toQColor(const OmarchyColor &color) {
    return {color.r, color.g, color.b};
}

QVariantList toRoleList(const OmarchySnapshot &snapshot) {
    QVariantList roles;
    // `roles` is a fixed-size C array from the FFI struct; `index` only ever runs to
    // OMARCHY_ROLE_COUNT.
    for (size_t index = 0; index < OMARCHY_ROLE_COUNT; ++index) {
        // NOLINTNEXTLINE(cppcoreguidelines-pro-bounds-constant-array-index)
        const OmarchyRole &role = snapshot.roles[index];

        QVariantMap entry;
        entry["name"] = QString::fromUtf8(omarchy_role_name(index));
        entry["color"] = toQColor(role.color);
        entry["onMain"] = toQColor(role.content_main);
        entry["onSecondary"] = toQColor(role.content_secondary);
        entry["onDisabled"] = toQColor(role.content_disabled);
        roles.append(entry);
    }
    return roles;
}

QVariantMap toShade(const QString &label, const OmarchyShade &shade) {
    QVariantMap entry;
    entry["label"] = label;
    entry["color"] = toQColor(shade.color);
    entry["onMain"] = toQColor(shade.content);
    return entry;
}

// The colors the theme declares, each as the family of three shades it reads as. A color the theme
// left out keeps its slot in the snapshot, and is dropped here so QML receives a list with nothing
// to skip.
QVariantList toPaletteList(const OmarchySnapshot &snapshot) {
    QVariantList palette;
    // `colors` is a fixed-size C array from the FFI struct; `index` only ever runs to
    // OMARCHY_COLOR_COUNT.
    for (size_t index = 0; index < OMARCHY_COLOR_COUNT; ++index) {
        // NOLINTNEXTLINE(cppcoreguidelines-pro-bounds-constant-array-index)
        const OmarchyColorFamily &family = snapshot.colors[index];
        if (!family.declared) {
            continue;
        }

        QVariantMap entry;
        entry["name"] = QString::fromUtf8(omarchy_color_name(index));
        entry["shades"] = QVariantList{
            toShade("Original", family.original),
            toShade("Dark", family.dark),
            toShade("Bright", family.bright),
        };
        palette.append(entry);
    }
    return palette;
}

QVariantList toRepresentationNameList() {
    QVariantList names;
    for (size_t index = 0; index < OMARCHY_REPRESENTATION_COUNT; ++index) {
        names.append(QString::fromUtf8(omarchy_representation_name(index)));
    }
    return names;
}

} // namespace

ThemeBridge::ThemeBridge(QObject *parent) : QObject(parent) {
    std::array<char, ERROR_BUFFER_SIZE> errorBuffer{};
    // NOLINTNEXTLINE(cppcoreguidelines-pro-bounds-array-to-pointer-decay) — the C ABI takes a
    // plain `char*` buffer; decaying `errorBuffer` into one is the whole point of passing it.
    m_watcher = omarchy_watch(nullptr, errorBuffer.data(), errorBuffer.size());

    if (m_watcher == nullptr) {
        m_startupError = QString::fromUtf8(errorBuffer.data());
        return;
    }

    refresh();

    m_notifier = new QSocketNotifier(omarchy_signal_fd(m_watcher), QSocketNotifier::Read, this);
    connect(m_notifier, &QSocketNotifier::activated, this, &ThemeBridge::onSignalReadable);
}

ThemeBridge::~ThemeBridge() {
    omarchy_free(m_watcher);
}

QVariantList ThemeBridge::roles() const {
    return m_roles;
}

QVariantList ThemeBridge::palette() const {
    return m_palette;
}

bool ThemeBridge::dark() const {
    return m_dark;
}

QString ThemeBridge::startupError() const {
    return m_startupError;
}

// A Q_PROPERTY READ function has to be an instance method for Qt's meta-object system, even one
// that reads no instance state.
// NOLINTNEXTLINE(readability-convert-member-functions-to-static)
QVariantList ThemeBridge::representationNames() const {
    return toRepresentationNameList();
}

int ThemeBridge::representation() const {
    return m_representation;
}

void ThemeBridge::setRepresentation(int representation) {
    if (representation == m_representation) {
        return;
    }

    m_representation = representation;
    emit representationChanged();

    if (m_watcher != nullptr) {
        refresh();
    }
}

void ThemeBridge::onSignalReadable() {
    // Drain every byte queued on the fd before polling — several changes can coalesce between two
    // wakeups, and only the newest one still matters.
    std::array<char, 64> discard{};
    const auto fd = static_cast<int>(m_notifier->socket());
    while (read(fd, discard.data(), discard.size()) > 0) {
    }

    OmarchySnapshot snapshot{};
    if (omarchy_poll_changed(m_watcher, static_cast<size_t>(m_representation), &snapshot)) {
        applySnapshot(snapshot);
    }
}

void ThemeBridge::refresh() {
    OmarchySnapshot snapshot{};
    omarchy_current(m_watcher, static_cast<size_t>(m_representation), &snapshot);
    applySnapshot(snapshot);
}

void ThemeBridge::applySnapshot(const OmarchySnapshot &snapshot) {
    m_roles = toRoleList(snapshot);
    m_palette = toPaletteList(snapshot);
    m_dark = snapshot.dark;
    emit themeChanged();
}
