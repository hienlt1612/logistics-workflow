import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "."

Rectangle {
    id: dashboard
    color: Theme.bgLight

    property var allShipments: []
    property var summary: null
    signal shipmentSelected(var shipment)

    function statusColor(status) {
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

    function statusLabel(status) {
        return status.replace(/_/g, " ")
    }

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: Theme.spacingLg
        spacing: Theme.spacingLg

        // ── HEADER ──
        RowLayout {
            Layout.fillWidth: true
            Text {
                text: "Dashboard"
                font.pixelSize: Theme.fontSizeH1
                font.bold: true
                color: Theme.textPrimary
            }
            Item { Layout.fillWidth: true }
            Text {
                text: summary ? summary.total + " shipments" : ""
                font.pixelSize: Theme.fontSizeBody
                color: Theme.textSecondary
            }
        }

        // ── STATUS CARDS ──
        RowLayout {
            Layout.fillWidth: true
            spacing: Theme.spacingMd

            Repeater {
                model: [
                    { label: "DRAFT", key: "draft", color: Theme.statusDraft, icon: "📝" },
                    { label: "DOCUMENTS", key: "documents", color: Theme.statusDocuments, icon: "📄" },
                    { label: "CUSTOMS", key: "customs", color: Theme.statusCustoms, icon: "🛃" },
                    { label: "CHECKLIST", key: "checklist", color: Theme.statusChecklist, icon: "✅" },
                    { label: "TELEX", key: "telex", color: Theme.statusTelex, icon: "📨" }
                ]

                delegate: Rectangle {
                    Layout.fillWidth: true
                    Layout.preferredHeight: 80
                    color: Theme.bgWhite
                    radius: Theme.radiusMd

                    Rectangle {
                        anchors.left: parent.left
                        anchors.top: parent.top
                        anchors.bottom: parent.bottom
                        width: 4
                        color: modelData.color
                        radius: 2
                    }

                    ColumnLayout {
                        anchors.fill: parent
                        anchors.margins: Theme.spacingMd
                        spacing: 2

                        RowLayout {
                            spacing: Theme.spacingSm
                            Text { text: modelData.icon; font.pixelSize: 16 }
                            Text {
                                text: modelData.label
                                font.pixelSize: Theme.fontSizeSmall
                                font.bold: true
                                color: Theme.textSecondary
                            }
                        }
                        Text {
                            text: summary ? (summary[modelData.key] || 0) : 0
                            font.pixelSize: 28
                            font.bold: true
                            color: modelData.color
                        }
                    }
                }
            }
        }

        // ── SHIPMENT TABLE HEADER ──
        RowLayout {
            Layout.fillWidth: true
            Text { text: "All Shipments"; font.pixelSize: Theme.fontSizeH2; font.bold: true; color: Theme.textPrimary }
            Item { Layout.fillWidth: true }
        }

        // ── TABLE ──
        Rectangle {
            Layout.fillWidth: true
            Layout.fillHeight: true
            color: Theme.bgWhite
            radius: Theme.radiusMd

            ListView {
                id: listView
                anchors.fill: parent
                anchors.margins: 1
                clip: true
                model: allShipments || []
                spacing: 0

                // Header row
                header: Rectangle {
                    width: listView.width
                    height: 40
                    color: "#F8F9FA"
                    z: 2

                    RowLayout {
                        anchors.fill: parent
                        anchors.leftMargin: Theme.spacingMd
                        anchors.rightMargin: Theme.spacingMd
                        spacing: 0

                        Text { Layout.preferredWidth: 110; text: "Ref"; font.pixelSize: 11; font.bold: true; color: Theme.textSecondary }
                        Text { Layout.preferredWidth: 130; text: "Buyer"; font.pixelSize: 11; font.bold: true; color: Theme.textSecondary }
                        Text { Layout.preferredWidth: 90; text: "Shipping Line"; font.pixelSize: 11; font.bold: true; color: Theme.textSecondary }
                        Text { Layout.preferredWidth: 100; text: "Status"; font.pixelSize: 11; font.bold: true; color: Theme.textSecondary }
                        Text { Layout.preferredWidth: 36; text: "BL"; font.pixelSize: 11; font.bold: true; color: Theme.textSecondary }
                        Text { Layout.preferredWidth: 36; text: "CO"; font.pixelSize: 11; font.bold: true; color: Theme.textSecondary }
                        Text { Layout.preferredWidth: 44; text: "Phyto"; font.pixelSize: 11; font.bold: true; color: Theme.textSecondary }
                        Text { Layout.preferredWidth: 52; text: "Charges"; font.pixelSize: 11; font.bold: true; color: Theme.textSecondary }
                        Text { Layout.preferredWidth: 44; text: "Docs"; font.pixelSize: 11; font.bold: true; color: Theme.textSecondary }
                        Text { Layout.preferredWidth: 44; text: "Telex"; font.pixelSize: 11; font.bold: true; color: Theme.textSecondary }
                        Item { Layout.fillWidth: true }
                    }
                }

                delegate: Rectangle {
                    width: listView.width
                    height: 44
                    color: index % 2 === 0 ? Theme.bgWhite : "#FAFBFC"

                    MouseArea {
                        anchors.fill: parent
                        cursorShape: Qt.PointingHandCursor
                        onClicked: dashboard.shipmentSelected(modelData)

                        RowLayout {
                            anchors.fill: parent
                            anchors.leftMargin: Theme.spacingMd
                            anchors.rightMargin: Theme.spacingMd
                            spacing: 0

                            Text {
                                Layout.preferredWidth: 110
                                text: modelData.shipment_ref || ""
                                font.pixelSize: Theme.fontSizeBody
                                font.bold: true
                                color: Theme.managerColor
                                elide: Text.ElideRight
                            }
                            Text {
                                Layout.preferredWidth: 130
                                text: modelData.buyer_name || "-"
                                font.pixelSize: Theme.fontSizeBody
                                color: Theme.textPrimary
                                elide: Text.ElideRight
                            }
                            Text {
                                Layout.preferredWidth: 90
                                text: modelData.shipping_line || "-"
                                font.pixelSize: Theme.fontSizeSmall
                                color: Theme.textSecondary
                                elide: Text.ElideRight
                            }
                            // Status badge
                            Rectangle {
                                Layout.preferredWidth: 90
                                Layout.preferredHeight: 24
                                radius: 12
                                color: dashboard.statusColor(modelData.status)

                                Text {
                                    anchors.centerIn: parent
                                    text: dashboard.statusLabel(modelData.status)
                                    font.pixelSize: 11
                                    font.bold: true
                                    color: Theme.textWhite
                                }
                            }
                            // Checklist indicators
                            CheckDot { Layout.preferredWidth: 36; active: modelData.bl_received }
                            CheckDot { Layout.preferredWidth: 36; active: modelData.co_received }
                            CheckDot { Layout.preferredWidth: 44; active: modelData.phyto_received }
                            CheckDot { Layout.preferredWidth: 52; active: modelData.charges_paid }
                            CheckDot { Layout.preferredWidth: 44; active: modelData.docs_confirmed }
                            CheckDot { Layout.preferredWidth: 44; active: modelData.telex_released }
                            Item { Layout.fillWidth: true }
                        }
                    }
                }

                // Empty state
                Rectangle {
                    anchors.fill: parent
                    color: "transparent"
                    visible: listView.count === 0
                    z: 1
                    Text {
                        anchors.centerIn: parent
                        text: "No shipments yet. Create one from the sidebar."
                        font.pixelSize: Theme.fontSizeBody
                        color: Theme.textSecondary
                    }
                }
            }
        }
    }

    // ── CheckDot component ──
    component CheckDot: Rectangle {
        property bool active: false
        Layout.preferredHeight: 24
        color: "transparent"

        Rectangle {
            anchors.centerIn: parent
            width: 20; height: 20
            radius: 10
            color: active ? "#27AE60" : "transparent"
            border.width: active ? 0 : 1.5
            border.color: active ? "transparent" : "#D0D0D0"

            Text {
                anchors.centerIn: parent
                text: active ? "✓" : ""
                font.pixelSize: 11
                font.bold: true
                color: Theme.textWhite
                visible: active
            }
        }
    }
}
