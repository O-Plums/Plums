use futures::future::{Future, result};
use actix_web::{HttpResponse, Error, AsyncResponder};

pub type FutureResponse = Box<Future<Item = HttpResponse, Error = Error>>;