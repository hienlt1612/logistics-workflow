import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "."

Rectangle {
    id: step2; radius: 8; color: "#FFFFFF"
    property var shipment: null; signal saveRequested(var formData)
    implicitHeight: formLayout.implicitHeight + headerBar.height + 48

    property string f_shipper: ""; property string f_consignee: ""; property string f_etd: ""
    property string f_invoice: ""; property string f_inv_date: ""; property string f_total: ""
    property string f_drafts: ""; property string f_bl: ""

    function build() { return { shipper_name:f_shipper, consignee_name:f_consignee, etd:f_etd, invoice_number:f_invoice, invoice_date:f_inv_date, total_value_usd:f_total, drafts_date:f_drafts, bill_of_lading:f_bl } }

    onShipmentChanged: { if(shipment){ f_shipper=shipment.shipper_name||""; f_consignee=shipment.consignee_name||""; f_etd=shipment.etd||""; f_invoice=shipment.invoice_number||""; f_inv_date=shipment.invoice_date||""; f_total=shipment.total_value_usd?String(shipment.total_value_usd):""; f_drafts=shipment.drafts_date||""; f_bl=shipment.bill_of_lading||"" } }

    Rectangle { id: headerBar; anchors.left: parent.left; anchors.right: parent.right; anchors.top: parent.top
        height: 44; color: "#3498DB"; radius: 8
        Rectangle { anchors.left: parent.left; anchors.right: parent.right; anchors.bottom: parent.bottom; height: 8; color: "#3498DB" }
        RowLayout { anchors.fill: parent; anchors.leftMargin: 16; spacing: 8
            Text { text: "STEP 2: DRAFT DOCUMENTATION"; font.pixelSize: 18; font.bold: true; color: "#FFFFFF" }
            Rectangle { Layout.preferredWidth: mgr.implicitWidth+12; Layout.preferredHeight: 22; radius: 11; color: "#40FFFFFF"
                Text { id: mgr; anchors.centerIn: parent; text: "MANAGER"; font.pixelSize: 12; font.bold: true; color: "#FFFFFF" } } } }

    ColumnLayout { id: formLayout; anchors.top: headerBar.bottom; anchors.left: parent.left; anchors.right: parent.right
        anchors.margins: 16; anchors.topMargin: 24; spacing: 8
        GridLayout { columns: 2; columnSpacing: 16; rowSpacing: 8; Layout.fillWidth: true
            LabeledField { label: "Shipper / Exporter *"; placeholder: "e.g., Hung Phat"; text: f_shipper; onFieldTextChanged: f_shipper=text }
            LabeledField { label: "Consignee *"; placeholder: "e.g., Kotor"; text: f_consignee; onFieldTextChanged: f_consignee=text }
            DateField { label: "ETD *"; dateText: f_etd; onDatePicked: f_etd=date }
            LabeledField { label: "Invoice and date *"; placeholder: "e.g., Inv 50-1"; text: f_invoice; onFieldTextChanged: f_invoice=text }
            LabeledField { label: "Total value USD *"; placeholder: "e.g., 333324"; text: f_total; onFieldTextChanged: f_total=text }
            DateField { label: "Invoice date"; dateText: f_inv_date; onDatePicked: f_inv_date=date }
            DateField { label: "Date of drafts"; dateText: f_drafts; onDatePicked: f_drafts=date }
            LabeledField { label: "Bill of lading #"; placeholder: "e.g., YSD354354"; text: f_bl; onFieldTextChanged: f_bl=text } }
        RowLayout { Layout.fillWidth: true; Item { Layout.fillWidth: true }
            Button { Layout.preferredWidth: 180; Layout.preferredHeight: 40; text: "Save & Continue"
                enabled: f_shipper!=="" && f_consignee!=="" && f_etd!=="" && f_invoice!=="" && f_total!==""
                background: Rectangle { color: parent.enabled ? (parent.hovered?"#2980B9":"#3498DB") : "#B0B0B0"; radius: 4 }
                contentItem: Text { text: parent.text; color: "#FFFFFF"; font.pixelSize: 14; font.bold: true; horizontalAlignment: Text.AlignHCenter; verticalAlignment: Text.AlignVCenter }
                onClicked: step2.saveRequested(step2.build()) } } }

    component LabeledField: ColumnLayout { property string label: ""; property string placeholder: ""; property alias text: tf.text
        signal fieldTextChanged(string t); Layout.fillWidth: true; spacing: 4
        Text { text: label; font.pixelSize: 14; font.bold: true; color: "#2C3E50" }
        TextField { id: tf; Layout.fillWidth: true; Layout.preferredHeight: 36; placeholderText: placeholder; font.pixelSize: 14
            background: Rectangle { color: "#F5F5F5"; radius: 4; border.color: "#D0D0D0"; border.width: 1 }
            onTextChanged: parent.fieldTextChanged(text) } }

    component DateField: ColumnLayout { property string label: ""; property string dateText: ""; signal datePicked(string date)
        Layout.fillWidth: true; spacing: 4
        Text { text: label; font.pixelSize: 14; font.bold: true; color: "#2C3E50" }
        RowLayout { Layout.fillWidth: true; spacing: 4
            TextField { id: df; Layout.fillWidth: true; Layout.preferredHeight: 36; placeholderText: "YYYY-MM-DD"; font.pixelSize: 14; text: dateText
                background: Rectangle { color: "#F5F5F5"; radius: 4; border.color: "#D0D0D0"; border.width: 1 } }
            Button { Layout.preferredWidth: 36; Layout.preferredHeight: 36; text: "📅"
                background: Rectangle { color: parent.hovered ? "#E0E0E0" : "#F5F5F5"; radius: 4; border.color: "#D0D0D0" }
                onClicked: cal.open(); CalendarPopup { id: cal; onDateSelected: function(d){ df.text=d; datePicked(d) } } } } }
}
