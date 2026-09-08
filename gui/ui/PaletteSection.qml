import QtQuick

// The Palette section: one row per color the theme declares, carrying the family of three it reads
// as — the color itself, its dark shade and its bright one, in that order. Keyboard-navigable like
// the other sections, with the current row framed in the theme's own focus color.
Item {
    id: _root

    // One entry per color: { name, shades: [{ label, color, onMain }, …] }.
    property var colors: []
    property color focusColor
    property color secondaryColor

    implicitHeight: _rows.height

    ListView {
        id: _rows

        anchors.left: parent.left
        anchors.right: parent.right
        // A view only lays out what fits in it, so its own content height is zero while it has no
        // height to fill. The rows are counted instead, at the height each one takes.
        height: count * rowHeight

        readonly property int rowHeight: 76

        activeFocusOnTab: true
        keyNavigationEnabled: true
        // The page scrolls as a whole; a list that scrolled on its own would swallow the wheel.
        interactive: false
        model: _root.colors

        delegate: Item {
            id: _row

            required property var modelData
            required property int index

            readonly property bool current: ListView.isCurrentItem

            width: _rows.width
            height: _rows.rowHeight

            Text {
                id: _name

                anchors.left: parent.left
                anchors.verticalCenter: parent.verticalCenter
                width: 96
                text: _row.modelData.name
                color: _root.secondaryColor
                font.pixelSize: 14
                font.bold: true
            }

            Row {
                anchors.left: _name.right
                anchors.right: parent.right
                anchors.verticalCenter: parent.verticalCenter
                spacing: 8

                Repeater {
                    model: _row.modelData.shades

                    Rectangle {
                        id: _shade

                        required property var modelData

                        width: (parent.width - 16) / 3
                        height: 60
                        radius: 8
                        color: modelData.color
                        // The ring marks the section holding the keyboard, so it follows the list's
                        // focus rather than lighting up on every row at once.
                        border.width: _row.current && _rows.activeFocus ? 3 : 1
                        border.color: _row.current && _rows.activeFocus
                            ? _root.focusColor
                            : _root.secondaryColor

                        Text {
                            anchors.left: parent.left
                            anchors.leftMargin: 12
                            anchors.top: parent.top
                            anchors.topMargin: 10
                            text: _shade.modelData.label
                            color: _shade.modelData.onMain
                            font.pixelSize: 12
                            font.bold: true
                        }

                        Text {
                            anchors.left: parent.left
                            anchors.leftMargin: 12
                            anchors.bottom: parent.bottom
                            anchors.bottomMargin: 10
                            text: _shade.modelData.color
                            color: _shade.modelData.onMain
                            font.pixelSize: 11
                        }
                    }
                }
            }

            MouseArea {
                anchors.fill: parent
                onClicked: _rows.currentIndex = _row.index
            }
        }
    }
}
