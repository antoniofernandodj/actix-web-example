use actix_web::guard::{Guard, GuardContext};


pub struct JWTGuard;
impl Guard for JWTGuard {
    fn check(&self, req: &GuardContext) -> bool {
        if let Some(logado) = req.head().headers().get("Logado") {
            if let Ok(v) = logado.to_str() {
                return v == "sim"
            }
        }
        return false
    }
}