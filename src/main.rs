use dashmap::DashMap;
use rand::{thread_rng, Rng};
use rocket::{
    response::{
        status::{BadRequest, NotFound},
        Redirect,
    },
    State,
};

#[macro_use]
extern crate rocket;

#[post("/api/shorten?<url>")]
fn shorten(url: String, state: &State<DashMap<u32, String>>) -> Result<String, BadRequest<&str>> {
    if url.is_empty() {
        Err(BadRequest(Some("URL is empty!")))
    } else {
        let key: u32 = thread_rng().gen();
        state.insert(key, url);
        Ok(key.to_string())
    }
}

#[get("/<key>")]
fn redirect(key: u32, state: &State<DashMap<u32, String>>) -> Result<Redirect, NotFound<&str>> {
    state
        .get(&key)
        .map(|url| Redirect::to(url.clone()))
        .ok_or(NotFound("Invalid or expired link!"))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(DashMap::<u32, String>::new())
        .mount("/", routes![shorten, redirect])
}

#[cfg(test)]
mod tests {
    use rocket::{http::Status, local::blocking::Client};

    use super::rocket;

    #[test]
    fn valid_requests() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.post("/api/shorten?url=https://duck.com").dispatch();
        assert_eq!(response.status(), Status::Ok);

        let key: u32 = response
            .into_string()
            .expect("body")
            .parse()
            .expect("valid u32");

        let response = client.get(format!("/{}", key)).dispatch();

        assert_eq!(response.status(), Status::SeeOther);

        let redirect = response
            .headers()
            .get_one("Location")
            .expect("location header");

        assert_eq!(redirect, "https://duck.com")
    }

    #[test]
    fn empty_url() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.post("/api/shorten?url=").dispatch();
        assert_eq!(response.status(), Status::BadRequest);
    }

    #[test]
    fn invalid_url() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.post("/123").dispatch();
        assert_eq!(response.status(), Status::NotFound);
    }
}
