use crate::api;
use actix_web::{
    dev::{
        forward_ready,
        Service,
        ServiceRequest,
        ServiceResponse,
        Transform
    },
    web,
    Error,
    Scope
};
use futures::FutureExt;
use rand::{thread_rng, Rng};
use std::future::{ready, Ready};
use futures_util::future::LocalBoxFuture;


pub fn get_scope() -> Scope {

    web::scope("/users")
        .route(
            "/",
            web::post()
                    // .guard(api::security::guards::JWTGuard)
                    .to(api::users::services::create)
        )

        .route(
            "/",
            web::get()
                    .guard(api::security::guards::JWTGuard)
                    .to(api::users::services::get_all)
        )

        .route(
            "/{uuid}",
            web::get()
                    .guard(api::security::guards::JWTGuard)
                    .to(api::users::services::get_one)
        )


        .route(
            "/{uuid}",
            web::delete()
                    .guard(api::security::guards::JWTGuard)
                    .to(api::users::services::delete)
        )

        .route(
            "/{uuid}",
            web::put()
                    .guard(api::security::guards::JWTGuard)
                    .to(api::users::services::update)
        )

        // .route(
        //     "/login",
        //     post()
        //             .to(api::users::services::login)
        // )

        .service(
            web::resource("/login")
            .wrap(Auth)
            .route(web::post().to(api::users::services::login))
        )


}


pub struct Auth;

impl
<NextServiceType, BodyType>
Transform<NextServiceType, ServiceRequest> for Auth
where
    NextServiceType: Service<
        ServiceRequest,
        Response = ServiceResponse<BodyType>,
        Error = Error
    >,
    NextServiceType::Future: 'static,
    BodyType: 'static,
{
    type Response = ServiceResponse<BodyType>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<NextServiceType>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: NextServiceType) -> Self::Future {

        ready(
            Ok::<
                AuthMiddleware<NextServiceType>, ()
            >(AuthMiddleware { service })
        )
    }
}

pub struct AuthMiddleware<NextServiceType> {
    service: NextServiceType,
}

impl<NextServiceType, BodyType> Service<ServiceRequest> for AuthMiddleware<NextServiceType>
where
    NextServiceType: Service<
        ServiceRequest,
        Response = ServiceResponse<BodyType>,
        Error = Error
    >,

    NextServiceType::Future: 'static,
    BodyType: 'static,
{
    type Response = ServiceResponse<BodyType>;
    type Error = Error;
    type Future = LocalBoxFuture<
        'static,
        Result<Self::Response, Self::Error>
    >;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {

        let auth = req
        .headers()
        .get(actix_web::http::header::AUTHORIZATION);

        if auth.is_none() {

            // let http_res = HttpResponse::Unauthorized().finish();
            // let (http_req, _) = req.into_parts();
            // let res = ServiceResponse::new(http_req, http_res);

            let err = actix_web::error::ErrorUnauthorized(
                "Unauthorized"
            );
            return (async move { Err(err) }).boxed_local();
        }

        let fut: <
            NextServiceType as Service<ServiceRequest>
        >::Future = self.service.call(req);

        Box::pin(async move {
            let res: ServiceResponse<BodyType> = fut.await?;
            let hm = res.headers();

            let auth = authenticate(hm).await;

            if auth {
                Ok(res)
            } else {
                Err(actix_web::error::ErrorUnauthorized("Unauthorized"))
            }

        })
    }
}


async fn authenticate(_hm: &actix_web::http::header::HeaderMap) -> bool {
    let mut rng: rand::prelude::ThreadRng = thread_rng();
    rng.gen_bool(0.5 as f64)
}