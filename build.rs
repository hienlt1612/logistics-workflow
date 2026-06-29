#[cfg(feature = "gui")]
use cxx_qt_build::{CxxQtBuilder, QmlModule};
#[cfg(feature = "gui")]
use std::path::Path;

fn main() {
    #[cfg(feature = "gui")]
    {
        let qml_files: &[&Path] = &[
            Path::new("qml/MainWindow.qml"),
            Path::new("qml/CalendarPopup.qml"),
            Path::new("qml/DashboardTab.qml"),
            Path::new("qml/ShipmentSidebar.qml"),
            Path::new("qml/WorkflowProgress.qml"),
            Path::new("qml/Step1Create.qml"),
            Path::new("qml/Step2Draft.qml"),
            Path::new("qml/Step3Customs.qml"),
            Path::new("qml/Step4Checklist.qml"),
            Path::new("qml/Theme.qml"),
        ];

        CxxQtBuilder::new()
            .qrc("resources/qml.qrc")
            .qml_module(QmlModule {
                uri: "com.logistics.workflow",
                rust_files: &[] as &[&Path],
                qml_files,
                ..Default::default()
            })
            .build();
    }
}
