use nickel_postgres::PostgresRequestExtensions;

use session::{Session, CookieSession};
use nickel::*;
use std::collections::HashMap;
use std::io::Read;
use ServerData;
use form_handler;
use hashaus;
pub fn login_router()->router::router::Router<ServerData>
{
    let mut login: router::router::Router<ServerData> = Nickel::router();
    login.get("/login", middleware!{|req, mut res| <ServerData>
        match*CookieSession::get_mut(req, &mut res)
        {
            Some(_) => *CookieSession::get_mut(req, &mut res) = None,
            _ => {
        
        let mut data = HashMap::new();
        data.insert("science", "course");
        return res.render("assets/login.html", &data);}
        }
        "<html><body><script>document.location.href = '/'</script></body></html>"
    });
    return login;
}
pub fn validation_router()->router::router::Router<ServerData>{
        let mut valid: router::router::Router<ServerData> = Nickel::router();
        valid.post("/valid", middleware!{|req, mut res|//, db|
            let mut form_data = String::new();
            req.origin.read_to_string(&mut form_data).unwrap();
            let form = form_handler::handle_form(&mut form_data);
            let u = (form.get("kayttaja").unwrap().to_string(),form.get("salasana").unwrap().to_string());
            let conn = req.db_conn();
            let stmt = conn.prepare("SELECT * FROM kayttaja WHERE 
                                    kayttajanimi = $1").unwrap();
            let mut b: bool = false;
            for row in stmt.query(&[&u.0.clone()]).unwrap(){
                let suola: String = row.get("suola");
                let tiiviste: i64 = row.get("salasana");
                if hashaus((suola+&u.1).to_string()) == tiiviste as u64
                {
                    b = true;
                    break;
                }
            }
            let a =if b == true {
                *CookieSession::get_mut(req, &mut res) = Some(u.0);
                
                "<html><body><script>document.location.href = '/'</script></body></html>"
            }
            //Testailuja, palauttaa TEST-käyttäjän. Use wisely.
            else if b == false{
                *CookieSession::get_mut(req, &mut res) = Some("TEST".to_string());
                
                "<html><body><script>document.location.href = '/'</script></body></html>"
            }
            else{
                "<html><body><script>document.location.href = '/login'</script></body></html>"
            };
            a

        
        });
        return valid;
} 
