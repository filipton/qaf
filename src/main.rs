use actix_web::{App, HttpServer};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
        //SERVICES
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

