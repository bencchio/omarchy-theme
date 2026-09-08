import QtQuick

// Derives every chrome color of the reference application from the roles the bridge resolves, so
// the whole frame — not just the swatches — follows the active theme and its chosen representation.
QtObject {
    id: _root

    // The resolved roles, each a map of { name, color, onMain, onSecondary, onDisabled }.
    property var roles: []

    // A property whose name starts with "on" followed by a capital reads as a signal handler, so
    // the binding written next to it is silently dropped and the color stays black. Every content
    // color here is named for the weight it carries, the way secondaryOnCanvas already was.
    readonly property color canvas: _role("Background").color
    readonly property color mainOnCanvas: _role("Background").onMain
    readonly property color secondaryOnCanvas: _role("Background").onSecondary
    readonly property color disabledOnCanvas: _role("Background").onDisabled

    readonly property color surface: _role("Elevated").color
    readonly property color mainOnSurface: _role("Elevated").onMain

    readonly property color well: _role("Recessed").color
    readonly property color wellOn: _role("Recessed").onMain

    readonly property color primary: _role("Primary").color
    readonly property color mainOnPrimary: _role("Primary").onMain

    readonly property color focus: _role("Focus").color
    readonly property color error: _role("Error").color
    readonly property color mainOnError: _role("Error").onMain

    function _role(name) {
        for (const entry of roles) {
            if (entry.name === name) {
                return entry;
            }
        }
        return _blank(name);
    }

    // A theme that could not be read leaves no roles behind, and the screen that explains why still
    // has to be legible — so every color below falls back rather than failing. This is the only
    // place in the application where a color does not come from the theme.
    function _blank(name) {
        return {
            color: name === "Error" ? "#c05050" : "#1c1c1c",
            onMain: "#e6e6e6",
            onSecondary: "#e6e6e6",
            onDisabled: "#e6e6e6"
        };
    }
}
