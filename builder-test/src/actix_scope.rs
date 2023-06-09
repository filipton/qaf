//THIS FILE IS AUTOGENERATED, DO NOT EDIT
use actix_web::web;

#[path = "pages"]
pub mod pages {
    pub mod another;
    pub mod test;
    pub mod inner {
        pub mod test2;
        pub mod nested {
            pub mod siema;
        }
    }
    pub mod outer {
        pub mod si;
    }
}

pub fn generated_scope() -> actix_web::Scope {
    web::scope("")
        .service(pages::test::get_tests)
        .service(pages::test::add_test)
        .service(pages::another::get_test2_index)
        .service(
            web::scope("inner")
                .service(pages::inner::test2::get_test2_index)
                .service(
                    web::scope("nested").service(pages::inner::nested::siema::get_kurwa_index),
                ),
        )
        .service(web::scope("outer").service(pages::outer::si::get_test2_index))
}
