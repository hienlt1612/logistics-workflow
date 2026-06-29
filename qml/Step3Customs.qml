import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "."

Rectangle {
    id: step3; radius: 8; color: "#FFFFFF"
    property var shipment: null; signal saveRequested(var formData)
    implicitHeight: formLayout.implicitHeight + headerBar.height + 48

    property string f_date: ""; property string f_number: ""; property string f_status: ""

    function build() { return { customs_date:f_date, customs_number:f_number, customs_status:f_status } }

    onShipmentChanged: { if(shipment){ f_date=shipment.customs_date||""; f_number=shipment.customs_number||""; f_status=shipment.customs_status||"" } }

    Rectangle { id: headerBar; anchors.left: parent.left; anchors.right: parent.right; anchors.top: parent.top
        height: 44; color: "#F39C12"; radius: 8
        Rectangle { anchors.left: parent.left; anchors.right: parent.right; anchors.bottom: parent.bottom; height: 8; color: "#F39C12" }
        RowLayout { anchors.fill: parent; anchors.leftMargin: 16; spacing: 8
            Text { text: "STEP 3: CUSTOMS CLEARANCE"; font.pixelSize: 18; font.bold: true; color: "#FFFFFF" }
            Rectangle { Layout.preferredWidth: acct.implicitWidth+12; Layout.preferredHeight: 22; radius: 11; color: "#40FFFFFF"
                Text { id: acct; anchors.centerIn: parent; text: "ACCOUNTING"; font.pixelSize: 12; font.bold: true; color: "#FFFFFF" } } } }

    ColumnLayout { id: formLayout; anchors.top: headerBar.bottom; anchors.left: parent.left; anchors.right: parent.right
        anchors.margins: 16; anchors.topMargin: 24; spacing: 8
        GridLayout { columns: 2; columnSpacing: 16; rowSpacing: 8; Layout.fillWidth: true
            DateField { label: "Date of customs clearance *"; dateText: f_date; onDatePicked: f_date=date }
            LabeledField { label: "Number of customs clearance *"; placeholder: "e.g., TKHQ364545"; text: f_number; onFieldTextChanged: f_number=text }
            ColumnLayout { Layout.fillWidth: true; spacing: 4
                Text { text: "Status of customs clearance *"; font.pixelSize: 14; font.bold: true; color: "#2C3E50" }
                ComboBox { id: combo; Layout.fillWidth: true; Layout.preferredHeight: 36; model: ["Red","Yellow","Green"]
                    currentIndex: { var s = f_status.toLowerCase(); if(s==="red")return 0; if(s==="yellow")return 1; if(s==="green")return 2; return -1 }
                    background: Rectangle { color: "#F5F5F5"; radius: 4; border.color: "#D0D0D0" }
                    onCurrentTextChanged: f_status = currentText } } }
        RowLayout { Layout.fillWidth: true; Item { Layout.fillWidth: true }
            Button { Layout.preferredWidth: 180; Layout.preferredHeight: 40; text: "Save & Continue"
                enabled: f_date!=="" && f_number!=="" && f_status!==""
                background: Rectangle { color: parent.enabled ? (parent.hovered?"#E67E22":"#F39C12") : "#B0B0B0"; radius: 4 }
                contentItem: Text { text: parent.text; color: "#FFFFFF"; font.pixelSize: 14; font.bold: true; horizontalAlignment: Text.AlignHCenter; verticalAlignment: Text.AlignVCenter }
                onClicked: step3.saveRequested(step3.build()) } } }

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
