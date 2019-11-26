#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

mod number_crunching {
    #[get("/hcf/<x>/<y>")]
    pub(crate) fn highest_common_factor(x: u128, y: u128) -> String {
        hcf_string(x, y)
    }

    #[get("/hcf/<x>/<y>", rank = 2)]
    pub(crate) fn highest_common_factor_signed(x: i128, y: i128) -> String {
        hcf_string(x, y)
    }

    #[get("/lcm/<x>/<y>")]
    pub(crate) fn lowest_common_multiple(x: u128, y: u128) -> String {
        lcm_string(x, y)
    }

    #[get("/lcm/<x>/<y>", rank = 2)]
    pub(crate) fn lowest_common_multiple_signed(x: i128, y: i128) -> String {
        lcm_string(x, y)
    }

    fn hcf_string<T>(x: T, y: T) -> String
    where
        T: num_integer::Integer + std::string::ToString,
    {
        num_integer::gcd(x, y).to_string()
    }

    fn lcm_string<T>(x: T, y: T) -> String
    where
        T: num_integer::Integer + std::string::ToString,
    {
        num_integer::lcm(x, y).to_string()
    }
}

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![
                index,
                number_crunching::highest_common_factor,
                number_crunching::lowest_common_multiple,
                number_crunching::highest_common_factor_signed,
                number_crunching::lowest_common_multiple_signed,
            ],
        )
        .launch();
}
