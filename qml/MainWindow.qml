import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "."

ApplicationWindow {
    id: root
    visible: true
    width: 1280
    height: 800
    title: "Logistics Workflow"
    color: Theme.bgLight

    property var selectedShipment: null
    property int selectedShipmentId: -1
    property var allShipments: []
    property var summary: null
    property bool dataLoaded: false
    property int refreshCounter: 0

    Component.onCompleted: loadData()

    // ── QML → Rust bridge via HTTP ──
    function sendCommand(action, data, id, field, value) {
        var cmd = { action: action }
        if (data !== undefined) cmd.data = data
        if (id !== undefined) cmd.id = id
        if (field !== undefined) cmd.field = field
        if (value !== undefined) cmd.value = value
        var xhr = new XMLHttpRequest()
        xhr.open("POST", "http://127.0.0.1:19876/", false)  // synchronous
        xhr.setRequestHeader("Content-Type", "application/json")
        try {
            xhr.send(JSON.stringify(cmd))
            var resp = JSON.parse(xhr.responseText)
            if (resp.ok) {
                refreshData()
                return resp.shipment
            } else {
                console.warn("Command failed:", resp.error)
            }
        } catch(e) {
            console.warn("Command error:", e)
        }
        return null
    }

    // Auto-refresh: poll AppData every second
    Timer {
        interval: 1000
        running: true
        repeat: true
        onTriggered: {
            loadData()
            refreshCounter++
        }
    }

    function loadData() {
        try {
            if (typeof AppData !== "undefined" && AppData.allShipmentsJson) {
                allShipments = JSON.parse(AppData.allShipmentsJson)
                dataLoaded = true
            }
            if (typeof AppData !== "undefined" && AppData.summaryJson) {
                summary = JSON.parse(AppData.summaryJson)
            }
        } catch(e) {}
    }

    function selectShipment(shipment) {
        selectedShipment = shipment
        selectedShipmentId = shipment ? shipment.id : -1
        viewStack.currentIndex = shipment ? 1 : 0
    }

    function showDashboard() {
        selectedShipment = null; selectedShipmentId = -1; viewStack.currentIndex = 0
    }

    function refreshData() {
        loadData()
        if (selectedShipmentId > 0) {
            var found = allShipments.find(function(s) { return s.id === selectedShipmentId })
            if (found) selectedShipment = found
        }
    }

    RowLayout {
        anchors.fill: parent; spacing: 0

        ShipmentSidebar {
            id: sidebar
            Layout.preferredWidth: 260; Layout.fillHeight: true
            allShipments: root.allShipments
            onShipmentSelected: function(s) { root.selectShipment(s) }
            onRefreshRequested: root.refreshData()
            onDashboardRequested: root.showDashboard()
            onCreateShipment: {
                var s = root.sendCommand("create_shipment", {
                    sc_po_id: "", sc_po_date: "", sc_po_by: "", buyer_name: "",
                    booking_number: "", shipping_line: "", origin_port: "",
                    warehouse_loc: "", loading_plan: ""
                })
                if (s) root.selectShipment(s)
            }
        }

        StackLayout {
            id: viewStack; Layout.fillWidth: true; Layout.fillHeight: true; currentIndex: 0

            DashboardTab {
                allShipments: root.allShipments; summary: root.summary
                onShipmentSelected: function(s) { root.selectShipment(s) }
            }

            Rectangle {
                color: Theme.bgLight
                ColumnLayout {
                    anchors.fill: parent; anchors.margins: Theme.spacingMd; spacing: Theme.spacingMd

                    RowLayout {
                        Layout.fillWidth: true
                        Button { text: "← Dashboard"; flat: true; font.pixelSize: Theme.fontSizeSmall; onClicked: root.showDashboard() }
                        Item { Layout.fillWidth: true }
                        Text { text: root.selectedShipment ? root.selectedShipment.shipment_ref : ""; font.pixelSize: Theme.fontSizeH2; font.bold: true; color: Theme.textPrimary }
                    }

                    WorkflowProgress {
                        Layout.fillWidth: true; Layout.preferredHeight: 60
                        currentStatus: root.selectedShipment ? root.selectedShipment.status : ""
                    }

                    Step1Create {
                        Layout.fillWidth: true
                        Layout.preferredHeight: visible ? implicitHeight : 0
                        visible: root.selectedShipment !== null
                        shipment: root.selectedShipment
                        onSaveRequested: function(formData) {
                            root.sendCommand(root.selectedShipmentId > 0 ? "update_shipment" : "create_shipment", formData, root.selectedShipmentId)
                        }
                    }

                    Step2Draft {
                        Layout.fillWidth: true
                        Layout.preferredHeight: visible ? implicitHeight : 0
                        visible: root.selectedShipment && ["DOCUMENTS_READY","CUSTOMS_CLEARED","CHECKLIST_IN_PROGRESS","COMPLETE","TELEX_RELEASED"].indexOf(root.selectedShipment.status) >= 0
                        shipment: root.selectedShipment
                        onSaveRequested: function(formData) {
                            root.sendCommand("update_shipment", formData, root.selectedShipmentId)
                        }
                    }

                    Step3Customs {
                        Layout.fillWidth: true
                        Layout.preferredHeight: visible ? implicitHeight : 0
                        visible: root.selectedShipment && ["CUSTOMS_CLEARED","CHECKLIST_IN_PROGRESS","COMPLETE","TELEX_RELEASED"].indexOf(root.selectedShipment.status) >= 0
                        shipment: root.selectedShipment
                        onSaveRequested: function(formData) {
                            root.sendCommand("update_shipment", formData, root.selectedShipmentId)
                        }
                    }

                    Step4Checklist {
                        Layout.fillWidth: true; Layout.fillHeight: true
                        visible: root.selectedShipment && ["CHECKLIST_IN_PROGRESS","COMPLETE","TELEX_RELEASED"].indexOf(root.selectedShipment.status) >= 0
                        shipment: root.selectedShipment
                        onSaveRequested: function(formData) {
                            root.sendCommand("update_shipment", formData, root.selectedShipmentId)
                        }
                        onToggleChecklist: function(field, value) {
                            root.sendCommand("toggle_checklist", null, root.selectedShipmentId, field, value)
                        }
                    }
                }
            }
        }
    }
}
