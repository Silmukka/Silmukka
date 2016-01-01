///Tässä tiedostossa luodaan routeri "/" polulle. Jos käyttäjä on kirjautuneena, näytetään omat
///tapahtumat, suosituimmat tapahtumat, lippukunnan tapahtumat ja hallinoimat tapahtumat. Jos ei ole, kerrotaan mikä on Silmukka ja ohjataan 
///rekisteröitymään
use nickel::{Nickel, HttpRouter};
use nickel::router::router::Router;
use session::{Session, CookieSession};
use std::collections::HashMap;
use ServerData;
pub fn route()->Router<ServerData>{
    let mut home: Router<ServerData> = Nickel::router();
    home.get("/", middleware!{|req, mut vastaus| 
        let mut palautetaan = HashMap::new();
        let logged = match *CookieSession::get_mut(req, &mut vastaus){
        Some(_) => true,
                  _ => false
        };
        if logged == false{
            palautetaan.insert("kirjaudu".to_string(), "Kirjaudu sisään".to_string());
            return vastaus.render("assets/out_index.html", &palautetaan);
        }
        else{
            let mut user = "/user/".to_string();
            // otetaan käyttäjänimi
            {let ref a: Option<String> = *CookieSession::get_mut(req, &mut vastaus);
                user.push_str(&(a.clone().unwrap()));}
            palautetaan.insert("kirjaudu".to_string(), "Kirjaudu ulos".to_string());
            palautetaan.insert("kayttaja".to_string(), user);
            return vastaus.render("assets/index.html", &palautetaan);
        }
    });
    return home;
}
