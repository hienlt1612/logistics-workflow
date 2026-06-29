import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "."

Rectangle {
    id: step1; radius: 8; color: "#FFFFFF"
    implicitHeight: formLayout.implicitHeight + headerBar.height + 48
    property var shipment: null
    signal saveRequested(var formData)

    // Reactive form fields — individual properties so bindings work
    property string f_sc_po_id: ""
    property string f_sc_po_date: ""
    property string f_sc_po_by: ""
    property string f_buyer_name: ""
    property string f_booking_number: ""
    property string f_shipping_line: ""
    property string f_origin_port: ""
    property string f_warehouse_loc: ""
    property string f_loading_plan: ""

    function buildFormData() {
        return {
            sc_po_id: f_sc_po_id, sc_po_date: f_sc_po_date, sc_po_by: f_sc_po_by,
            buyer_name: f_buyer_name, booking_number: f_booking_number,
            shipping_line: f_shipping_line, origin_port: f_origin_port,
            warehouse_loc: f_warehouse_loc, loading_plan: f_loading_plan
        }
    }

    onShipmentChanged: {
        if (shipment && shipment.id) {
            f_sc_po_id = shipment.sc_po_id || ""
            f_sc_po_date = shipment.sc_po_date || ""
            f_sc_po_by = shipment.sc_po_by || ""
            f_buyer_name = shipment.buyer_name || ""
            f_booking_number = shipment.booking_number || ""
            f_shipping_line = shipment.shipping_line || ""
            f_origin_port = shipment.origin_port || ""
            f_warehouse_loc = shipment.warehouse_loc || ""
            f_loading_plan = shipment.loading_plan || ""
        }
    }

    Rectangle { id: headerBar; anchors.left: parent.left; anchors.right: parent.right; anchors.top: parent.top
        height: 44; color: "#E74C3C"; radius: 8
        Rectangle { anchors.left: parent.left; anchors.right: parent.right; anchors.bottom: parent.bottom; height: 8; color: "#E74C3C" }
        RowLayout { anchors.fill: parent; anchors.leftMargin: 16; spacing: 8
            Text { text: "STEP 1: CREATE SHIPMENT"; font.pixelSize: 18; font.bold: true; color: "#FFFFFF" }
            Rectangle { Layout.preferredWidth: badge.implicitWidth+12; Layout.preferredHeight: 22; radius: 11; color: "#40FFFFFF"
                Text { id: badge; anchors.centerIn: parent; text: "ADMIN"; font.pixelSize: 12; font.bold: true; color: "#FFFFFF" } } } }

    ColumnLayout { id: formLayout; anchors.top: headerBar.bottom; anchors.left: parent.left; anchors.right: parent.right
        anchors.margins: 16; anchors.topMargin: 24; spacing: 8

        GridLayout { columns: 2; columnSpacing: 16; rowSpacing: 8; Layout.fillWidth: true
            LabeledField { label: "ID of SC/PO *"; placeholder: "e.g., PO-2026-001"; text: f_sc_po_id; onFieldTextChanged: f_sc_po_id = text }
            DateField { label: "Date of SC/PO"; dateText: f_sc_po_date; onDatePicked: f_sc_po_date = date }
            LabeledField { label: "SC/PO made by"; placeholder: "e.g., Tuan"; text: f_sc_po_by; onFieldTextChanged: f_sc_po_by = text }
            LabeledField { label: "For Buyer *"; placeholder: "e.g., Element"; text: f_buyer_name; onFieldTextChanged: f_buyer_name = text }
            LabeledField { label: "Number of booking *"; placeholder: "e.g., YMLAN6546547123"; text: f_booking_number; onFieldTextChanged: f_booking_number = text }
            LabeledField { label: "Shipping Line *"; placeholder: "e.g., Yang Ming"; text: f_shipping_line; onFieldTextChanged: f_shipping_line = text }
            LabeledField { label: "Port of Loading *"; placeholder: "e.g., Haiphong"; text: f_origin_port; onFieldTextChanged: f_origin_port = text }
            LabeledField { label: "Place of Warehouse"; placeholder: "e.g., Phu Cu"; text: f_warehouse_loc; onFieldTextChanged: f_warehouse_loc = text } }

        ColumnLayout { Layout.fillWidth: true; spacing: 4
            Text { text: "Loading plan and loading descriptions"; font.pixelSize: 14; font.bold: true; color: "#2C3E50" }
            TextArea { Layout.fillWidth: true; Layout.preferredHeight: 80; placeholderText: "e.g., 2 K30, 2 M20"
                text: f_loading_plan; font.pixelSize: 14; wrapMode: TextArea.Wrap
                background: Rectangle { color: "#F5F5F5"; radius: 4; border.color: "#D0D0D0"; border.width: 1 }
                onTextChanged: f_loading_plan = text } }

        RowLayout { Layout.fillWidth: true; Layout.topMargin: 8; Item { Layout.fillWidth: true }
            Button { Layout.preferredWidth: 180; Layout.preferredHeight: 40
                text: shipment && shipment.id ? "Save Changes" : "Create & Continue"
                enabled: f_sc_po_id !== "" && f_buyer_name !== "" && f_booking_number !== "" && f_shipping_line !== "" && f_origin_port !== ""
                background: Rectangle { color: parent.enabled ? (parent.hovered?"#C0392B":"#E74C3C") : "#B0B0B0"; radius: 4 }
                contentItem: Text { text: parent.text; color: "#FFFFFF"; font.pixelSize: 14; font.bold: true
                    horizontalAlignment: Text.AlignHCenter; verticalAlignment: Text.AlignVCenter }
                onClicked: step1.saveRequested(step1.buildFormData()) } } }

    component LabeledField: ColumnLayout { property string label: ""; property string placeholder: ""; property alias text: tf.text
        signal fieldTextChanged(string t); Layout.fillWidth: true; spacing: 4
        Text { text: label; font.pixelSize: 14; font.bold: true; color: "#2C3E50" }
        TextField { id: tf; Layout.fillWidth: true; Layout.preferredHeight: 36; placeholderText: placeholder; font.pixelSize: 14
            background: Rectangle { color: "#F5F5F5"; radius: 4; border.color: "#D0D0D0"; border.width: 1 }
            onTextChanged: parent.fieldTextChanged(text) } }

    component DateField: ColumnLayout { property string label: ""; property string dateText: ""
        signal datePicked(string date); Layout.fillWidth: true; spacing: 4
        Text { text: label; font.pixelSize: 14; font.bold: true; color: "#2C3E50" }
        RowLayout { Layout.fillWidth: true; spacing: 4
            TextField { id: df; Layout.fillWidth: true; Layout.preferredHeight: 36
                placeholderText: "YYYY-MM-DD"; font.pixelSize: 14; text: dateText
                background: Rectangle { color: "#F5F5F5"; radius: 4; border.color: "#D0D0D0"; border.width: 1 } }
            Button { Layout.preferredWidth: 36; Layout.preferredHeight: 36; text: "📅"
                background: Rectangle { color: parent.hovered ? "#E0E0E0" : "#F5F5F5"; radius: 4; border.color: "#D0D0D0" }
                onClicked: calPopup.open()
                CalendarPopup { id: calPopup; onDateSelected: function(d){ df.text=d; datePicked(d) } } } } }
}
