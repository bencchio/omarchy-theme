import QtQuick

// A section title and its optional subtitle, laid out like a real app's page heading rather than a
// debug label: the title carries the theme's primary accent, the subtitle reads as secondary text.
Item {
    id: _root

    property string title
    property string subtitle: ""
    property color accentColor
    property color subtitleColor

    implicitHeight: _body.height

    Column {
        id: _body

        anchors.left: parent.left
        anchors.right: parent.right
        spacing: 4

        Rectangle {
            width: 32
            height: 4
            radius: 2
            color: _root.accentColor
        }

        Text {
            text: _root.title
            color: _root.accentColor
            font.pixelSize: 20
            font.bold: true
        }

        Text {
            visible: _root.subtitle.length > 0
            text: _root.subtitle
            color: _root.subtitleColor
            font.pixelSize: 13
        }
    }
}
