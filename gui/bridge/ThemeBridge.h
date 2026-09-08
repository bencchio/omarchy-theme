#pragma once

#include <QObject>
#include <QSocketNotifier>
#include <QString>
#include <QVariantList>

#include "omarchy_theme.h"

// Owns the Rust watcher and exposes the current theme to QML. Change notification runs through
// the watcher's own fd (a QSocketNotifier), not a polling timer — the fd only turns readable
// after a valid change, so there is nothing to poll between them.
class ThemeBridge : public QObject {
    Q_OBJECT
    Q_PROPERTY(QVariantList roles READ roles NOTIFY themeChanged)
    Q_PROPERTY(QVariantList palette READ palette NOTIFY themeChanged)
    Q_PROPERTY(bool dark READ dark NOTIFY themeChanged)
    Q_PROPERTY(QString startupError READ startupError CONSTANT)
    Q_PROPERTY(QVariantList representationNames READ representationNames CONSTANT)
    Q_PROPERTY(int representation READ representation WRITE setRepresentation NOTIFY
                   representationChanged)

public:
    explicit ThemeBridge(QObject *parent = nullptr);
    ~ThemeBridge() override;

    ThemeBridge(const ThemeBridge &) = delete;
    ThemeBridge &operator=(const ThemeBridge &) = delete;
    ThemeBridge(ThemeBridge &&) = delete;
    ThemeBridge &operator=(ThemeBridge &&) = delete;

    [[nodiscard]] QVariantList roles() const;
    [[nodiscard]] QVariantList palette() const;
    [[nodiscard]] bool dark() const;
    [[nodiscard]] QString startupError() const;
    [[nodiscard]] QVariantList representationNames() const;
    [[nodiscard]] int representation() const;
    void setRepresentation(int representation);

signals:
    void themeChanged();
    void representationChanged();

private slots:
    void onSignalReadable();

private: // NOLINT(readability-redundant-access-specifiers) — kept to set slots and state apart
    void applySnapshot(const OmarchySnapshot &snapshot);
    void refresh();

    OmarchyWatcher *m_watcher = nullptr;
    QSocketNotifier *m_notifier = nullptr;
    QVariantList m_roles;
    QVariantList m_palette;
    bool m_dark = false;
    QString m_startupError;
    int m_representation = 0;
};
