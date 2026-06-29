// Theme.qml — Design system: colors, fonts, spacing
// Will be implemented in Task 2.4
pragma Singleton
import QtQuick

QtObject {
    // Role colors
    readonly property color adminColor: "#E74C3C"
    readonly property color managerColor: "#3498DB"
    readonly property color accountingColor: "#F39C12"
    readonly property color logisticsColor: "#9B59B6"

    // Status colors
    readonly property color statusDraft: "#95A5A6"
    readonly property color statusDocuments: "#3498DB"
    readonly property color statusCustoms: "#F39C12"
    readonly property color statusChecklist: "#27AE60"
    readonly property color statusTelex: "#9B59B6"

    // Backgrounds
    readonly property color bgDark: "#1A1A2E"
    readonly property color bgLight: "#F5F5F5"
    readonly property color bgWhite: "#FFFFFF"
    readonly property color bgCard: "#FFFFFF"

    // Text
    readonly property color textPrimary: "#2C3E50"
    readonly property color textSecondary: "#7F8C8D"
    readonly property color textWhite: "#FFFFFF"

    // Font sizes
    readonly property int fontSizeH1: 24
    readonly property int fontSizeH2: 18
    readonly property int fontSizeBody: 14
    readonly property int fontSizeSmall: 12

    // Spacing
    readonly property int spacingXs: 4
    readonly property int spacingSm: 8
    readonly property int spacingMd: 16
    readonly property int spacingLg: 24
    readonly property int spacingXl: 32

    // Border radius
    readonly property int radiusSm: 4
    readonly property int radiusMd: 8
    readonly property int radiusLg: 12
}
