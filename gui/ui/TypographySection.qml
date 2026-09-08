import QtQuick

// The Typography section: a sample passage laid out on the theme's own content weights, so the
// hierarchy (H1 through H4, body, caption) reads at a glance. Purely presentational — it is text,
// not controls, so it stays out of the keyboard focus chain.
Item {
    id: _root

    property color onMain
    property color onSecondary
    property color onDisabled

    implicitHeight: _body.height

    Column {
        id: _body

        anchors.left: parent.left
        anchors.right: parent.right
        spacing: 12

        Text {
            text: "The quick brown fox"
            color: _root.onMain
            font.pixelSize: 34
            font.bold: true
        }

        Text {
            text: "jumps over the lazy dog"
            color: _root.onMain
            font.pixelSize: 26
            font.bold: true
        }

        Text {
            text: "and every color keeps its promise"
            color: _root.onMain
            font.pixelSize: 20
            font.bold: true
        }

        Text {
            text: "from the canvas to the caption"
            color: _root.onMain
            font.pixelSize: 16
            font.bold: true
        }

        Text {
            width: _root.width
            text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor "
                  + "incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis "
                  + "nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat."
            color: _root.onMain
            font.pixelSize: 14
            lineHeight: 1.5
            wrapMode: Text.WordWrap
        }

        Text {
            width: _root.width
            text: "Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu "
                  + "fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in "
                  + "culpa qui officia deserunt mollit anim id est laborum."
            color: _root.onSecondary
            font.pixelSize: 13
            lineHeight: 1.5
            wrapMode: Text.WordWrap
        }

        Text {
            text: "A disabled caption, still legible but clearly not actionable"
            color: _root.onDisabled
            font.pixelSize: 12
            font.italic: true
        }
    }
}
