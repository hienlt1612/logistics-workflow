#[cfg(feature = "gui")]
use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QString, QUrl};

fn main() {
    env_logger::init();

    let serve_only = !cfg!(feature = "gui") || std::env::args().any(|a| a == "--serve");

    log::info!("Starting Logistics Workflow v0.2.0 (gui={}, serve_only={serve_only})",
        cfg!(feature = "gui"));

    // Load config
    let cfg = logistics_workflow::config::Config::load().expect("Failed to load config.toml");

    // Init tokio runtime + database
    let rt = tokio::runtime::Runtime::new().expect("Failed to create tokio runtime");
    rt.block_on(async {
        logistics_workflow::db::init_pool(&cfg.db_url())
            .await
            .expect("Failed to connect to database");
        logistics_workflow::db::schema::ensure_schema(logistics_workflow::db::pool())
            .await
            .expect("Failed to create schema");
        logistics_workflow::db::seed::seed_if_empty(logistics_workflow::db::pool())
            .await
            .expect("Failed to seed data");
    });
    logistics_workflow::TOKIO_RT.set(rt).ok();

    // Start REST API server (always)
    logistics_workflow::bridge::start_command_server();

    #[cfg(feature = "gui")]
    if !serve_only {
        // Generate AppData.qml before Qt starts
        logistics_workflow::bridge::regenerate_appdata();

        // Launch Qt application
        let mut app = QGuiApplication::new();
        app.pin_mut().set_application_name(&QString::from("Logistics Workflow"));

        let mut engine = QQmlApplicationEngine::new();
        if let Ok(cwd) = std::env::current_dir() {
            let qml_main = cwd.join("qml").join("MainWindow.qml");
            if let Some(qml_str) = qml_main.to_str() {
                engine.pin_mut().load(&QUrl::from_local_file(&QString::from(qml_str)));
            }
        }

        let exit_code = app.pin_mut().exec();
        std::process::exit(exit_code);
    }

    #[cfg(not(feature = "gui"))]
    {
        log::info!("Web-only mode — API running on http://127.0.0.1:19876");
        log::info!("Start frontend: cd web && npm run dev");
    }

    #[cfg(feature = "gui")]
    if serve_only {
        log::info!("Server-only mode — API running on http://127.0.0.1:19876");
        log::info!("Open web/index.html or run: cd web && npm run dev");
    }

    // Keep alive
    loop {
        std::thread::sleep(std::time::Duration::from_secs(3600));
    }
}
