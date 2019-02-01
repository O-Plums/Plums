use futures::future::{ Future };
use actix_web::{ HttpResponse, Error };

pub type FutureResponse = Box<Future<Item = HttpResponse, Error = Error>>;