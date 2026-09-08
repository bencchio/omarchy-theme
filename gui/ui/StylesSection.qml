import QtQuick

// The Styles section: every role the theme resolves to as a card, one per role, showing its own
// color and the three weights of content that read on it. The selected card lights up in the
// theme's own focus color, and both the arrows and the mouse reach every card.
Item {
    id: _root

    property var roles: []
    property color focusColor

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
        cellHeight: 132
        activeFocusOnTab: true
        keyNavigationEnabled: true
        // The page scrolls as a whole; a grid that scrolled on its own would swallow the wheel.
        interactive: false
        model: _root.roles

        delegate: RoleCard {
            required property var modelData
            required property int index

            roleName: modelData.name
            roleColor: modelData.color
            onMain: modelData.onMain
            onSecondary: modelData.onSecondary
            onDisabled: modelData.onDisabled
            focusColor: _root.focusColor
            // The ring marks the section holding the keyboard, so it follows the grid's focus.
            highlighted: GridView.isCurrentItem && _grid.activeFocus

            onSelected: _grid.currentIndex = index
        }
    }
}
