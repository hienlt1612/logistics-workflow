import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "."

Rectangle {
    id: step4; radius: 8; color: "#FFFFFF"
    implicitHeight: formLayout.implicitHeight + headerBar.height + 48
    property var shipment: null
    signal saveRequested(var formData)
    signal toggleChecklist(string field, bool value)

    // Reactive properties for non-boolean fields
    property string f_pay_date: ""; property string f_pay_amt: ""; property string f_remain: ""
    property string f_orig_status: ""; property string f_orig_sent: ""

    // Booleans tracked separately (updated by toggles immediately)
    property bool b_bl: false; property bool b_charges: false; property bool b_co: false
    property bool b_phyto: false; property bool b_docs: false; property bool b_telex: false

    function buildSave() { return {
        prepayment_date: f_pay_date, prepayment_amt: f_pay_amt, remaining_amt: f_remain,
        originals_status: f_orig_status, originals_sent: f_orig_sent
    }}

    onShipmentChanged: { if(shipment){
        b_bl=shipment.bl_received||false; b_charges=shipment.charges_paid||false; b_co=shipment.co_received||false
        b_phyto=shipment.phyto_received||false; b_docs=shipment.docs_confirmed||false; b_telex=shipment.telex_released||false
        f_pay_date=shipment.prepayment_date||""; f_pay_amt=shipment.prepayment_amt?String(shipment.prepayment_amt):""
        f_remain=shipment.remaining_amt?String(shipment.remaining_amt):""; f_orig_status=shipment.originals_status||""
        f_orig_sent=shipment.originals_sent||""
    }}

    Rectangle { id: headerBar; anchors.left: parent.left; anchors.right: parent.right; anchors.top: parent.top
        height: 44; color: "#9B59B6"; radius: 8
        Rectangle { anchors.left: parent.left; anchors.right: parent.right; anchors.bottom: parent.bottom; height: 8; color: "#9B59B6" }
        RowLayout { anchors.fill: parent; anchors.leftMargin: 16; spacing: 8
            Text { text: "STEP 4: DOCUMENT CHECKLIST"; font.pixelSize: 18; font.bold: true; color: "#FFFFFF" }
            Rectangle { Layout.preferredWidth: badge.implicitWidth+12; Layout.preferredHeight: 22; radius: 11; color: "#40FFFFFF"
                Text { id: badge; anchors.centerIn: parent; text: "ALL ROLES"; font.pixelSize: 12; font.bold: true; color: "#FFFFFF" } } } }

    ColumnLayout { id: formLayout; anchors.top: headerBar.bottom; anchors.left: parent.left; anchors.right: parent.right
        anchors.margins: 16; anchors.topMargin: 24; spacing: 16

        SectionLabel { text: "DOCUMENTS"; accentColor: "#3498DB" }
        GridLayout { columns: 2; columnSpacing: 16; Layout.fillWidth: true
            BoolCheck { label: "Bill of Lading received?"; checked: b_bl; accentColor: "#3498DB"
                onToggled: function(v){ b_bl=v; step4.toggleChecklist("bl_received",v) } }
            BoolCheck { label: "Documents confirmed?"; checked: b_docs; accentColor: "#3498DB"
                onToggled: function(v){ b_docs=v; step4.toggleChecklist("docs_confirmed",v) } } }

        SectionLabel { text: "CHARGES / THC"; accentColor: "#F39C12" }
        BoolCheck { label: "Charges / THC paid?"; checked: b_charges; accentColor: "#F39C12"
            onToggled: function(v){ b_charges=v; step4.toggleChecklist("charges_paid",v) } }

        SectionLabel { text: "CERTIFICATES"; accentColor: "#9B59B6" }
        GridLayout { columns: 2; columnSpacing: 16; Layout.fillWidth: true
            BoolCheck { label: "CO received?"; checked: b_co; accentColor: "#9B59B6"
                onToggled: function(v){ b_co=v; step4.toggleChecklist("co_received",v) } }
            BoolCheck { label: "Phyto received?"; checked: b_phyto; accentColor: "#9B59B6"
                onToggled: function(v){ b_phyto=v; step4.toggleChecklist("phyto_received",v) } } }

        SectionLabel { text: "PAYMENT TRACKING"; accentColor: "#F39C12" }
        GridLayout { columns: 3; columnSpacing: 16; Layout.fillWidth: true
            LabeledField { label: "Payment Date"; placeholder: "YYYY-MM-DD"; text: f_pay_date; onFieldTextChanged: f_pay_date=text }
            LabeledField { label: "Prepayment Amount"; placeholder: "e.g., 3655"; text: f_pay_amt; onFieldTextChanged: f_pay_amt=text }
            LabeledField { label: "Remaining Amount"; placeholder: "e.g., 329669"; text: f_remain; onFieldTextChanged: f_remain=text } }

        SectionLabel { text: "DOCUMENTS ORIGINALS"; accentColor: "#3498DB" }
        GridLayout { columns: 2; columnSpacing: 16; Layout.fillWidth: true
            LabeledField { label: "Status of originals"; placeholder: "e.g., Sent"; text: f_orig_status; onFieldTextChanged: f_orig_status=text }
            LabeledField { label: "Date of sending"; placeholder: "YYYY-MM-DD"; text: f_orig_sent; onFieldTextChanged: f_orig_sent=text } }

        SectionLabel { text: "TELEX RELEASE — FINAL"; accentColor: "#9B59B6" }
        BoolCheck { label: "Telex Released (FINAL ACTION)"; checked: b_telex; accentColor: "#9B59B6"
            warningText: "Warning: This is the final action and cannot be undone."
            onToggled: function(v){ b_telex=v; step4.toggleChecklist("telex_released",v) } }

        RowLayout { Layout.fillWidth: true; Item { Layout.fillWidth: true }
            Button { Layout.preferredWidth: 160; Layout.preferredHeight: 40; text: "Save Checklist"
                background: Rectangle { color: parent.hovered ? "#7D3C98" : "#9B59B6"; radius: 4 }
                contentItem: Text { text: parent.text; color: "#FFFFFF"; font.pixelSize: 14; font.bold: true; horizontalAlignment: Text.AlignHCenter; verticalAlignment: Text.AlignVCenter }
                onClicked: step4.saveRequested(step4.buildSave()) } } }

    component SectionLabel: Rectangle { property string text: ""; property color accentColor: "#3498DB"
        Layout.fillWidth: true; Layout.preferredHeight: 28; color: Qt.lighter(accentColor, 1.7)
        RowLayout { anchors.fill: parent; anchors.leftMargin: 8; spacing: 8
            Rectangle { Layout.preferredWidth: 4; Layout.preferredHeight: 20; color: parent.parent.accentColor; radius: 2 }
            Text { text: parent.parent.text; font.pixelSize: 14; font.bold: true; color: parent.parent.accentColor } } }

    component BoolCheck: RowLayout { property string label: ""; property bool checked: false; property color accentColor: "#3498DB"; property string warningText: ""; signal toggled(bool v)
        Layout.fillWidth: true; spacing: 8
        CheckBox { id: cb; checked: parent.checked
            indicator: Rectangle { implicitWidth:24; implicitHeight:24; radius:4; border.color:cb.checked?parent.parent.accentColor:"#BDBDBD"; border.width:2; color:cb.checked?parent.parent.accentColor:"transparent"
                Text { anchors.centerIn:parent; text:cb.checked?"✓":""; color:"#FFFFFF"; font.pixelSize:14; font.bold:true } }
            onCheckedChanged: parent.toggled(checked) }
        ColumnLayout { spacing:2
            Text { text: label; font.pixelSize:14; color:checked?"#2C3E50":"#7F8C8D" }
            Text { text: warningText; font.pixelSize:12; color:"#E74C3C"; font.bold:true; visible:warningText!=="" } } }

    component LabeledField: ColumnLayout { property string label: ""; property string placeholder: ""; property alias text: tf.text; signal fieldTextChanged(string t)
        Layout.fillWidth: true; spacing:4
        Text { text: label; font.pixelSize:14; font.bold:true; color:"#2C3E50" }
        TextField { id: tf; Layout.fillWidth:true; Layout.preferredHeight:36; placeholderText:placeholder; font.pixelSize:14
            background: Rectangle { color:"#F5F5F5"; radius:4; border.color:"#D0D0D0"; border.width:1 }
            onTextChanged: parent.fieldTextChanged(text) } }
}
