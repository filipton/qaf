use actix_web::{App, HttpServer, web};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
        .service(web::scope("")
.service(web::scope("inner")
.service(pages::inner::test2::get_test2_index)
.service(web::scope("nested")
.service(pages::inner::nested::siema::get_kurwa_index)
)).service(pages::test::get_test_index)
.service(pages::test::get_post_index)
.service(web::scope("outer")
.service(pages::outer::si::get_test2_index)
).service(pages::another::get_test2_index)
)
    )
        .bind(("0.0.0.0", 8081))?
        .run()
        .await
}

pub mod pages { 
	pub mod inner { 
		pub mod test2; 
		pub mod nested { 
			pub mod siema; 
		} 
	} 
	pub mod test; 
	pub mod outer { 
		pub mod si; 
	} 
	pub mod another; 
} 

