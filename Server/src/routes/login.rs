use actix_web::{ Error, HttpResponse, FutureResponse, State, Path };
use futures::Future;
use models::{ AppState };
use models::users::{ CreateUser };
use crate::routes::actix_web::AsyncResponder;

pub fn Login((phone, state): (Path<String>, State<AppState>),) -> FutureResponse<HttpResponse> {
    println!("{:?}", phone);
    state.db
        .send(CreateUser { phone: phone.into_inner(), validation_code: 42 })
        .from_err()
        .and_then(|res| match res {
            Ok(user) => Ok(HttpResponse::Ok().json(user)),
            Err(_) => Ok(HttpResponse::InternalServerError().into())
        })
        .responder()
}