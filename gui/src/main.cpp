#include <QGuiApplication>
#include <QQmlApplicationEngine>
#include <QQmlContext>

#include "ThemeBridge.h"

int main(int argc, char *argv[]) {
    QGuiApplication app(argc, argv);

    ThemeBridge bridge;

    QQmlApplicationEngine engine;
    engine.rootContext()->setContextProperty("bridge", &bridge);
    engine.load(QUrl(QStringLiteral("qrc:/qt/qml/OmarchyShowcase/ui/Main.qml")));

    if (engine.rootObjects().isEmpty()) {
        return -1;
    }

    return QGuiApplication::exec();
}
