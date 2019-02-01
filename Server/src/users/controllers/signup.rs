use futures::future::{ Future, result };
use actix_web::{ AsyncResponder, HttpRequest, HttpResponse };

use crate::State;
use crate::DBstate;
use crate::utils::responses::{ FutureResponse };
use crate::users::model::{ New, Find };
use crate::utils::regex::{ check_new_user };


pub struct Signup {
    pub errors: Option<Vec<String>>
}

impl Signup {
    pub fn get(request: HttpRequest<State>) -> FutureResponse {
        let phone = request.query_string();
        if check_new_user(phone) {
            let v: Vec<&str> = phone.split("cphone=").collect();
            let v2 : Vec<&str> = v[1].split("&phone=").collect();
            let phone = v2[1].to_string();
            let phone_code = v2[0].to_string();
            let find = Find{ phone_code: phone_code.clone(), phone: phone.clone() };
            let DBstate(db) = request.state().db;
            match db.send(find).wait().unwrap().unwrap() {
                Some(user) => result(Ok(HttpResponse::Ok().json(user))).responder(),
                None => {
                    let data_user = db.send(New{ phone_code: phone_code, phone: phone, validation_code: 0 }).wait();
                    match data_user.unwrap() {
                        Ok(new_user) => result(Ok(HttpResponse::Ok().json(new_user))).responder(),
                        Err(err) => result(Ok(HttpResponse::BadRequest().json(err.to_string()))).responder()
                    }
                }
            }
        } else {
            result(Ok(HttpResponse::BadRequest().finish())).responder()
        }
    }
}