use actix_web::{get, post, web::{self, Json}, App, HttpResponse, HttpServer,middleware};
use serde::{Serialize,Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
//This is for the Response 
#[derive(Debug, Deserialize, Serialize)]
pub struct Response<T> {
    pub results: Vec<T>
}
pub struct Like<T>
{ 
    pub results:Vec<T>
}

//This for the tweet details
#[derive(Deserialize,Serialize,Debug)]
pub struct Details{ 
   pub id:String ,    
   pub  message:String,
}
 
 impl Details{ 
    pub fn new(message:String)->Self{ 
          Self{ 
            id:Uuid::new_v4().to_string(),
            message,
          }
    }
 }

 //This is for the Tweet Response 
 #[derive(Deserialize,Serialize,Debug)]
 pub struct Response1{ 
   pub tweet_Response:Option<String>,
 }

 impl Response1{ 
    pub fn tweet_Res(&self)->Option<Details>{ 
     match  &self.tweet_Response{ 
            Some(tweet_Response)=>Some(Details::new(tweet_Response.to_string())),
            None=>None,
     }
    }
 }

 //This is for the likes part 
 #[derive(Deserialize,Serialize)]
 pub struct Likes {
    like:String,
    time_like:DateTime<Utc>
 }
 
 impl Likes{ 
    pub fn like()->Self{ 
           Self{ 
               like:Uuid::new_v4().to_string(),
               time_like:Utc::now(),
           }
    }
 }
 #[get("/tweets")]
 pub async fn get_data()->HttpResponse{ 
      let tweets:Response<Details>=Response {results:vec![]};

      HttpResponse::Ok()
      .content_type("application/json")
      .json(tweets)
 }

 #[post("/tweets")]
 pub async fn post_data(req:Json<Response1>)->HttpResponse{ 
    HttpResponse::Created()
    .content_type("application/json")
    .json(req.tweet_Res())
 }

#[get("/tweets/{id}")]
pub async fn get_data1(path :web::Path<(String,)>)->HttpResponse
{ 
    let stream:Option<Details>=None;

    match stream{ 
        Some(tweet)=> 
        HttpResponse::Ok()
        .content_type("application/json")
        .json(tweet),
        None=>
        HttpResponse::NoContent()
        .content_type("application/json")
        .await
        .unwrap(),
    }
}

#[actix_web::delete("/tweets/{id}")]
async fn delete(_path:web::Path<(String,)>)->HttpResponse
{  
   HttpResponse::NoContent()
    .content_type("application/json")
     .await
    .unwrap()
}
#[get("/tweets/{id}/likes")]
pub async fn like_list(_path: web::Path<(String,)>) -> HttpResponse {
        let likes:Response<Likes> = Response {results: vec![] };

    HttpResponse::Ok()
        .content_type("application/json")
        .json(likes)
}


#[post("/tweets/{id}/likes")]
pub async fn plus_one(_path:web:: Path<(String,)>) -> HttpResponse {
    // TODO add one like to a tweet
    let like = Likes::like();

    HttpResponse::Created()
        .content_type("application/json")
        .json(like)
}


#[actix_web::delete("/tweets/{id}/likes")]
pub async fn minus_one(path:web:: Path<(String,)>) -> HttpResponse {
    
    HttpResponse::NoContent()
        .content_type("application/json")
        .await
        .unwrap()
}
#[actix_web::main]
async fn main()->std::io::Result<()>
{   //env variables
   std:: env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(||
     {
        App::new()
        .wrap(middleware::Logger::default())
        // register HTTP requests handlers
        .service(get_data)
        .service(post_data)
        .service(get_data1)
        .service(delete)
        .service(like_list)
        .service(plus_one)
        .service(minus_one)
     })
     .bind(("127.0.0.1",8080))? 
     .run()
     .await
}