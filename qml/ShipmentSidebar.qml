import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "."

Rectangle {
    id: sidebar
    color: Theme.bgDark

    property var allShipments: []
    signal shipmentSelected(var shipment)
    signal refreshRequested()
    signal dashboardRequested()
    signal createShipment()

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: Theme.spacingSm
        spacing: Theme.spacingSm

        // Dashboard button
        Button {
            Layout.fillWidth: true
            Layout.preferredHeight: 36
            text: "🏠 Dashboard"

            background: Rectangle {
                color: parent.hovered ? "#2A3A5C" : "transparent"
                radius: Theme.radiusSm
            }
            contentItem: Text {
                text: parent.text
                color: Theme.textWhite
                font.pixelSize: Theme.fontSizeBody
                verticalAlignment: Text.AlignVCenter
                leftPadding: Theme.spacingSm
            }
            onClicked: sidebar.dashboardRequested()
        }

        // Header
        Text {
            Layout.fillWidth: true
            text: "SHIPMENTS"
            font.pixelSize: Theme.fontSizeH2
            font.bold: true
            color: Theme.textWhite
            horizontalAlignment: Text.AlignHCenter
            topPadding: Theme.spacingSm
        }

        // New Shipment Button
        Button {
            id: newBtn
            Layout.fillWidth: true
            Layout.preferredHeight: 40
            text: "+ New Shipment"

            background: Rectangle {
                color: newBtn.hovered ? Theme.adminColor : "#C0392B"
                radius: Theme.radiusSm
            }
            contentItem: Text {
                text: newBtn.text
                color: Theme.textWhite
                font.pixelSize: Theme.fontSizeBody
                font.bold: true
                horizontalAlignment: Text.AlignHCenter
                verticalAlignment: Text.AlignVCenter
            }

            onClicked: sidebar.createShipment()
        }

        // Search/Filter
        TextField {
            id: searchField
            Layout.fillWidth: true
            Layout.preferredHeight: 36
            placeholderText: "Search shipments..."
            placeholderTextColor: Theme.textSecondary
            color: Theme.textWhite
            font.pixelSize: Theme.fontSizeSmall

            background: Rectangle {
                color: "#16213E"
                radius: Theme.radiusSm
                border.color: "#2A3A5C"
                border.width: 1
            }
        }

        // Status filter
        ComboBox {
            id: statusFilter
            Layout.fillWidth: true
            Layout.preferredHeight: 36
            model: ["All Statuses", "DRAFT", "DOCUMENTS_READY", "CUSTOMS_CLEARED", "CHECKLIST_IN_PROGRESS", "COMPLETE", "TELEX_RELEASED"]
            currentIndex: 0

            background: Rectangle {
                color: "#16213E"
                radius: Theme.radiusSm
                border.color: "#2A3A5C"
            }
            contentItem: Text {
                text: statusFilter.currentText
                color: Theme.textWhite
                font.pixelSize: Theme.fontSizeSmall
                verticalAlignment: Text.AlignVCenter
            }
        }

        // Shipment List
        ListView {
            id: listView
            Layout.fillWidth: true
            Layout.fillHeight: true
            clip: true
            spacing: Theme.spacingXs
            model: sidebar.filteredShipments

            delegate: Rectangle {
                width: listView.width
                height: 56
                color: shipmentSelected ? "#2A3A5C" : "transparent"
                radius: Theme.radiusSm

                property bool shipmentSelected: false
                property var shipment: modelData

                Component.onCompleted: {
                    // Highlight selected shipment
                    shipmentSelected = false  // will be set by MainWindow
                }

                MouseArea {
                    anchors.fill: parent
                    onClicked: {
                        sidebar.shipmentSelected(modelData)
                    }
                }

                RowLayout {
                    anchors.fill: parent
                    anchors.margins: Theme.spacingSm
                    spacing: Theme.spacingSm

                    // Status dot
                    Rectangle {
                        Layout.preferredWidth: 10
                        Layout.preferredHeight: 10
                        radius: 5
                        color: getStatusColor(modelData.status)
                    }

                    ColumnLayout {
                        Layout.fillWidth: true
                        spacing: 2

                        Text {
                            text: modelData.shipment_ref || "Unknown"
                            font.pixelSize: Theme.fontSizeBody
                            font.bold: true
                            color: Theme.textWhite
                        }
                        Text {
                            text: modelData.buyer_name || "No buyer"
                            font.pixelSize: Theme.fontSizeSmall
                            color: Theme.textSecondary
                        }
                    }

                    // Status badge
                    Rectangle {
                        Layout.preferredWidth: statusLabel.implicitWidth + 16
                        Layout.preferredHeight: 22
                        radius: Theme.radiusSm
                        color: getStatusBg(modelData.status)

                        Text {
                            id: statusLabel
                            anchors.centerIn: parent
                            text: modelData.status.replace("_", " ")
                            font.pixelSize: Theme.fontSizeSmall
                            font.bold: true
                            color: Theme.textWhite
                        }
                    }
                }
            }
        }

        // Filtered model
        property var filteredShipments: {
            var result = allShipments || []
            var search = searchField.text.toLowerCase()
            var statusIdx = statusFilter.currentIndex

            if (search) {
                result = result.filter(function(s) {
                    return (s.shipment_ref && s.shipment_ref.toLowerCase().indexOf(search) >= 0)
                        || (s.buyer_name && s.buyer_name.toLowerCase().indexOf(search) >= 0)
                })
            }

            if (statusIdx > 0) {
                var statuses = ["", "DRAFT", "DOCUMENTS_READY", "CUSTOMS_CLEARED", "CHECKLIST_IN_PROGRESS", "COMPLETE", "TELEX_RELEASED"]
                var target = statuses[statusIdx]
                result = result.filter(function(s) { return s.status === target })
            }

            return result
        }

        // Empty state
        Text {
            Layout.fillWidth: true
            Layout.alignment: Qt.AlignCenter
            text: allShipments.length === 0 ? "No shipments yet" : "No matching shipments"
            font.pixelSize: Theme.fontSizeSmall
            color: Theme.textSecondary
            horizontalAlignment: Text.AlignHCenter
            visible: (sidebar.filteredShipments || []).length === 0
        }
    }

    function getStatusColor(status) {
        switch(status) {
            case "DRAFT": return Theme.statusDraft
            case "DOCUMENTS_READY": return Theme.statusDocuments
            case "CUSTOMS_CLEARED": return Theme.statusCustoms
            case "CHECKLIST_IN_PROGRESS": return Theme.statusChecklist
            case "COMPLETE": return Theme.statusChecklist
            case "TELEX_RELEASED": return Theme.statusTelex
            default: return Theme.statusDraft
        }
    }

    function getStatusBg(status) {
        switch(status) {
            case "DRAFT": return Theme.statusDraft
            case "DOCUMENTS_READY": return Theme.statusDocuments
            case "CUSTOMS_CLEARED": return Theme.statusCustoms
            case "CHECKLIST_IN_PROGRESS": return Theme.statusChecklist
            case "COMPLETE": return Theme.statusChecklist
            case "TELEX_RELEASED": return Theme.statusTelex
            default: return Theme.statusDraft
        }
    }
}
