import QtQuick

// The Roles section: each of the ten roles COLOR resolves, as a plain swatch with its name, so
// the whole set shows at a glance. Keyboard-navigable like the styles grid, with the current
// swatch framed in the theme's own focus color.
Item {
    id: _root

    property var roles: []
    property color focusColor
    property color secondaryColor

    implicitHeight: _grid.height

    GridView {
        id: _grid

        anchors.left: parent.left
        anchors.right: parent.right
        // A view only lays out what fits in it, so its own content height is zero while it has no
        // height to fill. The rows are counted instead, over the columns the given width allows.
        readonly property int columns: Math.max(1, Math.floor(width / cellWidth))
        height: Math.ceil(count / columns) * cellHeight

        cellWidth: 220
        cellHeight: 88
        activeFocusOnTab: true
        keyNavigationEnabled: true
        // The page scrolls as a whole; a grid that scrolled on its own would swallow the wheel.
        interactive: false
        model: _root.roles

        delegate: Rectangle {
            id: _swatch

            required property var modelData
            required property int index

            // `isCurrentItem` is attached to the delegate, not to the view: read from the view it is
            // undefined, and the focus ring never draws.
            readonly property bool current: GridView.isCurrentItem

            width: 208
            height: 76
            radius: 8
            color: modelData.color
            // The ring marks the section holding the keyboard, so it follows the grid's focus.
            border.width: current && _grid.activeFocus ? 3 : 1
            border.color: current && _grid.activeFocus ? _root.focusColor : _root.secondaryColor

            // A hint of the content color along the top edge. It reads as a rule over the swatch,
            // not as a second field competing with it, so it stays thin.
            Rectangle {
                anchors.top: parent.top
                anchors.left: parent.left
                anchors.right: parent.right
                height: 6
                topLeftRadius: parent.radius
                topRightRadius: parent.radius
                color: modelData.onMain
                opacity: 0.35
            }

            Text {
                anchors.left: parent.left
                anchors.leftMargin: 12
                anchors.bottom: parent.bottom
                anchors.bottomMargin: 10
                text: _swatch.modelData.name
                color: modelData.onMain
                font.pixelSize: 14
                font.bold: true
            }

            Text {
                anchors.right: parent.right
                anchors.rightMargin: 12
                anchors.verticalCenter: parent.verticalCenter
                text: _swatch.modelData.color
                color: modelData.onMain
                font.pixelSize: 11
            }

            MouseArea {
                anchors.fill: parent
                onClicked: _grid.currentIndex = _swatch.index
            }
        }
    }
}
