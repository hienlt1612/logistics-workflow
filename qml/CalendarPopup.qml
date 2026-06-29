import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Popup {
    id: popup
    width: 280; height: 300
    closePolicy: Popup.CloseOnEscape | Popup.CloseOnPressOutside
    signal dateSelected(string date)

    property int viewYear: new Date().getFullYear()
    property int viewMonth: new Date().getMonth()

    function open() { popup.open() }

    function daysInMonth(y, m) { return new Date(y, m+1, 0).getDate() }
    function monthName(m) {
        var names = ["Jan","Feb","Mar","Apr","May","Jun","Jul","Aug","Sep","Oct","Nov","Dec"]
        return names[m]
    }
    function pad(n) { return n < 10 ? "0"+n : ""+n }

    ColumnLayout {
        anchors.fill: parent; spacing: 4

        // Month navigation
        RowLayout {
            Layout.fillWidth: true
            Button { text: "◀"; flat: true; onClicked: { if(viewMonth===0){viewMonth=11;viewYear--}else{viewMonth--} } }
            Text { Layout.fillWidth: true; text: monthName(viewMonth) + " " + viewYear
                font.pixelSize: 16; font.bold: true; horizontalAlignment: Text.AlignHCenter; color: "#2C3E50" }
            Button { text: "▶"; flat: true; onClicked: { if(viewMonth===11){viewMonth=0;viewYear++}else{viewMonth++} } }
        }

        // Day headers
        RowLayout { Layout.fillWidth: true
            Repeater { model: ["Mo","Tu","We","Th","Fr","Sa","Su"]
                delegate: Text { Layout.fillWidth: true; text: modelData; font.pixelSize: 11; font.bold: true
                    color: "#7F8C8D"; horizontalAlignment: Text.AlignHCenter } } }

        // Day grid
        GridLayout {
            Layout.fillWidth: true; columns: 7; rowSpacing: 2; columnSpacing: 2

            Repeater {
                model: {
                    var days = []
                    var first = new Date(viewYear, viewMonth, 1).getDay() || 7 // Mon=1..Sun=7
                    for (var i = 1; i < first; i++) days.push({ day: "", active: false })
                    var total = daysInMonth(viewYear, viewMonth)
                    var today = new Date()
                    for (var d = 1; d <= total; d++) {
                        days.push({
                            day: d,
                            active: true,
                            isToday: viewYear===today.getFullYear() && viewMonth===today.getMonth() && d===today.getDate()
                        })
                    }
                    return days
                }

                delegate: Rectangle {
                    Layout.preferredWidth: 32; Layout.preferredHeight: 32; radius: 16
                    color: modelData.active && modelData.isToday ? "#E74C3C" : "transparent"

                    Text {
                        anchors.centerIn: parent
                        text: modelData.day || ""
                        font.pixelSize: 13
                        font.bold: modelData.isToday
                        color: modelData.isToday ? "#FFFFFF" : "#2C3E50"
                        visible: modelData.active
                    }

                    MouseArea {
                        anchors.fill: parent
                        enabled: modelData.active
                        onClicked: {
                            var d = popup.pad(modelData.day)
                            var m = popup.pad(popup.viewMonth + 1)
                            var y = popup.viewYear
                            popup.dateSelected(y + "-" + m + "-" + d)
                            popup.close()
                        }
                    }
                }
            }
        }

        // Today + Clear
        RowLayout { Layout.fillWidth: true
            Item { Layout.fillWidth: true }
            Button { text: "Today"; flat: true; font.pixelSize: 12
                onClicked: { var t=new Date(); popup.dateSelected(t.getFullYear()+"-"+popup.pad(t.getMonth()+1)+"-"+popup.pad(t.getDate())); popup.close() } }
            Button { text: "Clear"; flat: true; font.pixelSize: 12; onClicked: { popup.dateSelected(""); popup.close() } }
        }
    }
}
