extern crate iron;
extern crate router;
extern crate urlencoded;

use iron::prelude::*;
use iron::status;
use router::Router;
use std::str::FromStr;
use urlencoded::UrlEncodedBody;

fn main() {
    let mut router = Router::new();

    router.get("/", get_form, "root");
    router.post("/gcd", post_gcd, "gcd");

    println!("Serveur en http://localhost:3000/");
    Iron::new(router).http("localhost:3000").unwrap();
}

fn get_form(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    response.set_mut(status::Ok);
    response.headers.set(iron::headers::ContentType(
        "text/html; charset=utf-8"
            .parse::<iron::mime::Mime>()
            .unwrap(),
    ));
    response.set_mut(
        r#"
        <title>Calculatrice PGCD</title>
        <form action="/gcd" method="post">
            <input type="text" name="n"/>
            <input type="text" name="n"/>
            <button type="submit">Calculer le PGCD</button>
        </form>
    "#,
    );

    Ok(response)
}

fn post_gcd(request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    let form_data = match request.get_ref::<UrlEncodedBody>() {
        Err(e) => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("Erreur dans l'analyze du formulaire {:?}\n", e));

            return Ok(response);
        }
        Ok(map) => map,
    };

    let unparsed_numbers = match form_data.get("n") {
        None => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("Pas de 'n' dans le formulaire\n"));

            return Ok(response);
        }
        Some(nums) => nums,
    };

    let mut nombres = Vec::new();
    for unparsed in unparsed_numbers {
        match u64::from_str(&unparsed) {
            Err(_) => {
                response.set_mut(status::BadRequest);
                response.set_mut(format!("Valeur non num. pour 'n' : {:?}\n", unparsed));

                return Ok(response);
            }
            Ok(n) => {
                nombres.push(n);
            }
        };
    }

    let mut d = nombres[0];
    for m in &nombres[1..] {
        d = gcd(d, *m);
    }

    response.set_mut(status::Ok);
    response.headers.set(iron::headers::ContentType(
        "text/html; charset=utf-8"
            .parse::<iron::mime::Mime>()
            .unwrap(),
    ));
    response.set_mut(format!("Le PGCD de {:?} est <b>{}</b>\n", nombres, d));
    Ok(response)
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}
