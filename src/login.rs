use nickel_postgres::PostgresRequestExtensions;
use session::{Session, CookieSession};
use nickel::*;
use std::collections::HashMap;
use std::io::Read;
use ServerData;
//Purkka, tulee muuttumaan
fn paskahash(hashattava: String)->String
{
        hashattava
}
pub fn login_router()->router::router::Router<ServerData>
{
    let mut login: router::router::Router<ServerData> = Nickel::router();
    login.get("/login", middleware!{|_, res|
        let mut data = HashMap::new();
        data.insert("science", "course");
        return res.render("assets/login.html", &data);
    });
    return login;
}
pub fn validation_router()->router::router::Router<ServerData>{
        let mut valid: router::router::Router<ServerData> = Nickel::router();
        valid.post("/valid", middleware!{|req, mut res|//, db|
            let mut form_data = String::new();
            req.origin.read_to_string(&mut form_data).unwrap();
            let mut u = (String::new(), String::new());
            //PURKKA ALKAA, yritetään keksiä paremp POST hallinta, kuten PHP:ssä
            form_data.remove(0);
            form_data.remove(0);
            for i in form_data.clone().chars(){
                if i == '&'
                {
                    form_data.remove(0);
                    form_data.remove(0);
                    form_data.remove(0);
                    break;
                }
                u.0.push(i);
                form_data.remove(0);
            }
            u.1 = form_data;
            //PURKKA LOPPUU
            let conn = req.db_conn();
            let stmt = conn.prepare("SELECT (suola, salasana) FROM kayttaja WHERE 
                                    kayttajanimi = $1").unwrap();
            let mut b: bool = false;
            for row in stmt.query(&[&u.0.clone()]).unwrap(){
                let suola: String = row.get(0);
                let tiiviste: String = row.get(1);
                if paskahash((suola+&u.1).to_string()) == tiiviste
                {
                    b = true;
                    break;
                }
            }
            let a =if b == false{
                *CookieSession::get_mut(req, &mut res) = Some(u.0);
                
                "<html><body><script>document.location.href = '/'</script></body></html>"
            }
            else{
                "<html><body><script>document.location.href = '/login'</script></body></html>"
            };
            a

        
        });
        return valid;
} 
