#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
use rocket_contrib::serve::StaticFiles;

mod farm_out_hard_work {
    use rocket::request::Form;
    use rocket::response::content;
    use futures::try_join;
    use failure::format_err;
    use old_futures::future::Future;

    #[derive(Debug, FromForm)]
    pub(crate) struct Input {
        x: isize,
        y: isize,
    }

    #[derive(Serialize)]
    struct Output {
        lcm: usize,
        hcf: usize,
    }

    impl Output {
        fn from_tuple(tuple: (String, String)) -> Self {
            Output {
                lcm: tuple.0.parse::<usize>().unwrap(),
                hcf: tuple.1.parse::<usize>().unwrap(),
            }
        }
    }

    #[get("/domath?<homework..>")]
    pub(crate) fn do_math(homework: Form<Input>) -> content::Json<String> {
        // We call out to our other server to find LCM & HCF
        // of the two numbers.
        let results = async {
            let lcm = reqwest::get(&format!("http://localhost:8000/lcm/{}/{}", homework.x, homework.y))
                .await?
                .error_for_status()?
                .text();

            let hcf = reqwest::get(&format!("http://localhost:8000/hcf/{}/{}", homework.x, homework.y))
                .await?
                .error_for_status()?
                .text();

            try_join!(lcm, hcf)
        };

        let mut rt = tokio::runtime::Runtime::new().unwrap();
        let results = Output::from_tuple(rt.block_on(results).unwrap());
        content::Json(serde_json::to_string(&results).unwrap())
    }

    #[get("/domath_old?<homework..>")]
    pub(crate) fn do_math_old(homework: Form<Input>) -> content::Json<String> {
        // We call out to our other server to find LCM & HCF
        // of the two numbers.
        let client = old_reqwest::r#async::Client::new();
        let text = |mut res: old_reqwest::r#async::Response| res.text();
        let lcm = client
                    .get(&format!("http://localhost:8000/lcm/{}/{}", homework.x, homework.y))
                    .send()
                    .and_then(|r| r.error_for_status())
                    .and_then(text)
                    .map_err(|_| format_err!("whoopsie!"));
        let hcf = client
                    .get(&format!("http://localhost:8000/hcf/{}/{}", homework.x, homework.y))
                    .send()
                    .and_then(|r| r.error_for_status())
                    .and_then(text)
                    .map_err(|_| format_err!("whoopsie!"));

	let results = lcm.join(hcf);

	let mut rt = old_tokio::runtime::Runtime::new().unwrap();
	let results = Output::from_tuple(rt.block_on(results).unwrap());
        content::Json(serde_json::to_string(&results).unwrap())
    }

}

fn main() {
    rocket::ignite()
        .mount("/", routes![farm_out_hard_work::do_math_old,farm_out_hard_work::do_math,])
	.mount("/hi", StaticFiles::from("static"))
        .launch();
}
