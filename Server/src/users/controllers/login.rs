use futures::future::{ Future, result };
use actix_web::{ AsyncResponder, HttpRequest, HttpResponse, Json, Error };

use crate::State;
use crate::utils::responses::{ FutureResponse };
use crate::users::model::{ New, Find };
use crate::utils::regex::{ check_new_user };


pub struct Login {
    pub errors: Option<Vec<String>>
}

impl Login {
    pub fn post((data, request) : (Json<New>, HttpRequest<State>)) -> FutureResponse {
        let query = &data;
        let find = Find{ phone_code: data.phone_code.clone(), phone: data.phone.clone()};
        request.state().db.send(find).from_err().and_then(move | res | match res {
            Ok(option_user) => {
                match option_user {
                    Some(user) => {
                        if user.validation_code == data.validation_code {
                            let mut modif = user.to_update();
                            modif.validation_code = 0;
                            match request.state().db.send(modif).wait() {
                                Ok(u) => Ok(HttpResponse::Ok().json(u.unwrap())),
                                Err(_) => Ok(HttpResponse::BadRequest().json("Update user fail".to_string()))
                            }
                        } else {
                            Ok(HttpResponse::BadRequest().json("fail validation user".to_string()))
                        }
                    },
                    None => Ok(HttpResponse::BadRequest().json("user not found".to_string()))
                }
            },
            Err(err) => Ok(HttpResponse::BadRequest().json(err.to_string()))
        }).responder()
    }
}