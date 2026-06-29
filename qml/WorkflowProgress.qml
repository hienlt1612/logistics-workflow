import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "."

Rectangle {
    id: progress
    color: Theme.bgWhite
    radius: Theme.radiusMd

    property string currentStatus: ""

    // Step definitions
    property var steps: [
        { label: "CREATE", role: "ADMIN", color: Theme.adminColor, status: "DRAFT" },
        { label: "DRAFT", role: "MANAGER", color: Theme.managerColor, status: "DOCUMENTS_READY" },
        { label: "CUSTOMS", role: "ACCT", color: Theme.accountingColor, status: "CUSTOMS_CLEARED" },
        { label: "CHECKLIST", role: "ALL", color: Theme.logisticsColor, status: "CHECKLIST_IN_PROGRESS" },
        { label: "TELEX", role: "FINAL", color: Theme.statusTelex, status: "TELEX_RELEASED" }
    ]

    property var statusOrder: ["DRAFT", "DOCUMENTS_READY", "CUSTOMS_CLEARED", "CHECKLIST_IN_PROGRESS", "COMPLETE", "TELEX_RELEASED"]

    function currentStepIndex() {
        if (!currentStatus) return 0
        var idx = statusOrder.indexOf(currentStatus)
        // Map CHECKLIST_IN_PROGRESS and COMPLETE to step 3
        if (currentStatus === "COMPLETE") return 3
        if (currentStatus === "CHECKLIST_IN_PROGRESS") return 3
        if (idx < 0) return 0
        // Map to 0-4 steps
        if (idx <= 0) return 0  // DRAFT
        if (idx <= 1) return 1  // DOCUMENTS_READY
        if (idx <= 2) return 2  // CUSTOMS_CLEARED
        if (idx <= 4) return 3  // CHECKLIST/COMPLETE
        return 4  // TELEX_RELEASED
    }

    RowLayout {
        anchors.fill: parent
        anchors.margins: Theme.spacingSm
        spacing: 0

        Repeater {
            model: steps
            delegate: Item {
                Layout.fillWidth: true
                Layout.fillHeight: true

                ColumnLayout {
                    anchors.centerIn: parent
                    spacing: 4

                    // Circle
                    Rectangle {
                        Layout.preferredWidth: 32
                        Layout.preferredHeight: 32
                        Layout.alignment: Qt.AlignHCenter
                        radius: 16
                        color: {
                            var currentIdx = progress.currentStepIndex()
                            if (index < currentIdx) return modelData.color
                            if (index === currentIdx) return modelData.color
                            return "#E0E0E0"
                        }
                        border.width: 2
                        border.color: {
                            var currentIdx = progress.currentStepIndex()
                            if (index <= currentIdx) return modelData.color
                            return "#BDBDBD"
                        }

                        Text {
                            anchors.centerIn: parent
                            text: {
                                var currentIdx = progress.currentStepIndex()
                                if (index < currentIdx) return "✓"
                                return String(index + 1)
                            }
                            color: {
                                var currentIdx = progress.currentStepIndex()
                                if (index <= currentIdx) return Theme.textWhite
                                return Theme.textSecondary
                            }
                            font.pixelSize: Theme.fontSizeSmall
                            font.bold: true
                        }
                    }

                    // Label
                    Text {
                        Layout.alignment: Qt.AlignHCenter
                        text: modelData.label
                        font.pixelSize: 10
                        font.bold: {
                            var currentIdx = progress.currentStepIndex()
                            return index === currentIdx
                        }
                        color: {
                            var currentIdx = progress.currentStepIndex()
                            if (index <= currentIdx) return modelData.color
                            return Theme.textSecondary
                        }
                    }

                    // Role badge
                    Rectangle {
                        Layout.alignment: Qt.AlignHCenter
                        Layout.preferredWidth: roleLabel.implicitWidth + 8
                        Layout.preferredHeight: 14
                        radius: 7
                        color: modelData.color
                        visible: {
                            var currentIdx = progress.currentStepIndex()
                            return index <= currentIdx
                        }

                        Text {
                            id: roleLabel
                            anchors.centerIn: parent
                            text: modelData.role
                            font.pixelSize: 8
                            color: Theme.textWhite
                            font.bold: true
                        }
                    }
                }

                // Connector line (except after last)
                Rectangle {
                    anchors.left: parent.horizontalCenter
                    anchors.leftMargin: 32
                    anchors.verticalCenter: parent.verticalCenter
                    width: parent.width / 2 - 16
                    height: 3
                    color: {
                        var currentIdx = progress.currentStepIndex()
                        if (index < currentIdx) return steps[index].color
                        return "#E0E0E0"
                    }
                    visible: index < 4
                }
            }
        }
    }
}
