import QtQuick

// The reference application: a real-looking window, not a debug panel. Every chrome color — canvas,
// surfaces, accents, the mode selector, the focus ring — comes from the theme the bridge resolves,
// so the whole frame (not just the swatches) follows a theme change and the chosen representation.
// The sections stack and scroll; Tab and the arrows reach every control, and the focused element is
// always distinguished by the theme's own Focus color.
Window {
    id: _root

    width: 760
    height: 640
    visible: true
    title: "omarchy-theme reference"
    color: _theme.canvas

    Theme {
        id: _theme
        roles: bridge.roles
    }

    // A theme that cannot be read surfaces the reason instead of a silent, empty window.
    Text {
        visible: bridge.startupError.length > 0
        anchors.centerIn: parent
        color: _theme.error
        text: "Could not read the active theme:\n" + bridge.startupError
        horizontalAlignment: Text.AlignHCenter
        font.pixelSize: 14
    }

    Column {
        id: _header

        anchors.top: parent.top
        anchors.left: parent.left
        anchors.right: parent.right
        anchors.margins: 24
        spacing: 8
        visible: bridge.startupError.length === 0

        Row {
            spacing: 8

            Rectangle {
                width: 12
                height: 12
                radius: 3
                anchors.verticalCenter: parent.verticalCenter
                color: _theme.primary
            }

            Text {
                text: "omarchy-theme"
                color: _theme.primary
                font.pixelSize: 22
                font.bold: true
            }
        }

        Text {
            text: "A reference application built on the active theme's colors"
            color: _theme.secondaryOnCanvas
            font.pixelSize: 13
        }

        ListView {
            id: _modes

            width: parent.width
            height: 44
            orientation: ListView.Horizontal
            spacing: 8
            focus: true
            activeFocusOnTab: true
            keyNavigationEnabled: true
            model: bridge.representationNames

            // A binding on currentIndex would be silently dropped the first time the user
            // navigates, since the view itself assigns to it imperatively — set the starting
            // value once instead, and keep the sync one-way from here to the bridge.
            Component.onCompleted: currentIndex = bridge.representation
            onCurrentIndexChanged: bridge.representation = currentIndex

            delegate: Rectangle {
                id: _tab

                required property string modelData
                required property int index

                // `isCurrentItem` is attached to the delegate, not to the view: read from the view
                // it is undefined, and the selected tab never stands out.
                readonly property bool current: ListView.isCurrentItem

                width: 132
                height: _modes.height
                radius: 6
                color: current ? _theme.primary : _theme.well
                // The ring marks the section holding the keyboard, so it follows the view's focus.
                border.width: current && _modes.activeFocus ? 2 : 0
                border.color: _theme.focus

                Text {
                    anchors.centerIn: parent
                    text: _tab.modelData
                    color: _tab.current ? _theme.mainOnPrimary : _theme.wellOn
                }

                MouseArea {
                    anchors.fill: parent
                    onClicked: _modes.currentIndex = _tab.index
                }
            }
        }
    }

    Flickable {
        id: _content

        anchors.top: _header.bottom
        anchors.topMargin: 24
        anchors.left: parent.left
        anchors.right: parent.right
        anchors.bottom: parent.bottom
        contentHeight: _sections.height
        clip: true
        boundsBehavior: Flickable.StopAtBounds
        visible: bridge.startupError.length === 0

        Column {
            id: _sections

            anchors.left: parent.left
            anchors.right: parent.right
            anchors.margins: 24
            spacing: 28

            SectionHeader {
                width: parent.width
                title: "Styles"
                subtitle: "Every semantic role, with the content that reads on it"
                accentColor: _theme.primary
                subtitleColor: _theme.secondaryOnCanvas
            }

            StylesSection {
                id: _stylesGrid

                width: parent.width
                roles: bridge.roles
                focusColor: _theme.focus
            }

            SectionHeader {
                width: parent.width
                title: "Typography"
                subtitle: "Sample text on the theme's content weights"
                accentColor: _theme.primary
                subtitleColor: _theme.secondaryOnCanvas
            }

            TypographySection {
                width: parent.width
                onMain: _theme.mainOnCanvas
                onSecondary: _theme.secondaryOnCanvas
                onDisabled: _theme.disabledOnCanvas
            }

            SectionHeader {
                width: parent.width
                title: "Roles"
                subtitle: "Every role the library resolves, one swatch each"
                accentColor: _theme.primary
                subtitleColor: _theme.secondaryOnCanvas
            }

            RolesSection {
                id: _rolesGrid

                width: parent.width
                roles: bridge.roles
                focusColor: _theme.focus
                secondaryColor: _theme.secondaryOnCanvas
            }

            SectionHeader {
                width: parent.width
                title: "Palette"
                subtitle: "The colors the theme itself declares"
                accentColor: _theme.primary
                subtitleColor: _theme.secondaryOnCanvas
            }

            PaletteSection {
                id: _paletteGrid

                width: parent.width
                colors: bridge.palette
                focusColor: _theme.focus
                secondaryColor: _theme.secondaryOnCanvas
            }
        }
    }
}
