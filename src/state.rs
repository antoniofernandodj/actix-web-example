use actix_web::web::Data;


#[allow(dead_code)]
pub struct AppState {
    pub app_name: String
}

impl AppState {
    pub fn new(app_name: String) -> Self {
        AppState {app_name: app_name}
    }
}

pub fn get_app_data() -> Data<AppState> {
    let state: AppState = AppState::new(
        "Actix Web".to_owned()
    );

    Data::new(state)
}
