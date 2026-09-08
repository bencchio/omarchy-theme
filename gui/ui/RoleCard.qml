import QtQuick

// One role of the active theme: its own color, and its three weights of content, with a border
// that lights up in the theme's own focus color when this is the selected card.
Rectangle {
    id: _root

    property string roleName
    property color roleColor
    property color onMain
    property color onSecondary
    property color onDisabled
    property color focusColor
    property bool highlighted

    signal selected

    width: 200
    height: 120
    radius: 8
    color: roleColor
    border.width: highlighted ? 3 : 0
    border.color: focusColor

    Column {
        anchors.fill: parent
        anchors.margins: 12
        spacing: 6

        Text {
            text: _root.roleName
            color: _root.onMain
            font.bold: true
        }
        Text {
            text: "Aa"
            color: _root.onMain
        }
        Text {
            text: "Aa"
            color: _root.onSecondary
        }
        Text {
            text: "Aa"
            color: _root.onDisabled
        }
    }

    MouseArea {
        anchors.fill: parent
        onClicked: _root.selected()
    }
}
