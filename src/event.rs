use nickel_postgres::PostgresRequestExtensions;
use session::{Session, CookieSession};
use nickel::*;
use std::collections::HashMap;
use postgres_array::array::Array;
use std::io::Read;
use ServerData;
use form_handler;
pub type Reititin = router::router::Router<ServerData>;
use parse;
pub fn create_event()->Reititin
{
    let mut creator: router::router::Router<ServerData> = Nickel::router();
    creator.get("/create_event", middleware!{ |req, mut res| <ServerData>
        let conn = req.db_conn();
        let stmt = conn.prepare("select ika from kayttaja where kayttajanimi = $1").unwrap();
        let a = "Foul play".to_string();
        let mut man = String::new();
        assert_eq!(man, String::new());
            {man= match *CookieSession::get_mut(req, &mut res){
                Some(ref name)=>name,             
                _ => &a
                }.to_string();
        }
        for row in stmt.query(&[&man]).unwrap()
        {
            let ik: String  = row.get("ika");
            let ika = &ik;
            if ika == ""
            {
                break;
            }
            
        }
        let mut data = HashMap::new();
        data.insert("Kirjaudu", "Kirjaudu ulos");
        return res.render("assets/creator.html", &data);

    });
    return creator;
}
fn parse_list(_str: &str)->Vec<String>
{
    let mut lista: Vec<String> = Vec::new();
    let mut temp = String::new();
    for chara in _str.chars()
    {
        if chara == '*'{
            lista.push(temp);   
            
            temp = String::new();
        }
        else{
            temp.push(chara);
        }
    }
    if temp != String::new() || temp != "".to_string()
    {
        lista.push(temp);
    }
    return lista;
}
pub fn creator()->Reititin
{
    let mut creator: Reititin = Nickel::router();
    creator.post("/event_create", middleware!{|req, mut res| <ServerData>
        let conn = req.db_conn();
        let mut man = String::new();
        let empty = String::new();
        assert_eq!(man, String::new());
            {man= match *CookieSession::get_mut(req, &mut res){
                Some(ref name)=>name,             
                _ => &empty
                }.to_string();
        }
        let mut form_data = String::new();
        req.origin.read_to_string(&mut form_data).unwrap();
        let stmt = conn.prepare("select id from kayttaja where kayttajanimi = $1").unwrap();
        let mut id: i32 = 0;
        for row in stmt.query(&[&man]).unwrap()
        {
            let cd: i32 = row.get("id");
            id+=cd;
        }
        let adminit = Array::from_vec(vec![id], 1);
        let form = form_handler::handle_form(&mut form_data);
        let lista = Array::from_vec(parse_list(&(form.get("lista").unwrap().to_string()+
                                                 "*Partiohuivi")),1); 
        let vlista = Array::from_vec(parse_list(form.get("vlista").unwrap()),1); 
        conn.execute("INSERT INTO tapahtuma (nimi, kuvaus, admins, lista, vartiokohtaiset_varusteet)
                     VALUES($1, $2, $3, $4, $5)",
        &[&(form.get("nimi").unwrap().to_string().replace("+", "_")), form.get("kuvaus").unwrap(),&adminit, &lista, &vlista]).unwrap(); 
        "<html><body><script>document.location.href = '/event/".to_string()+
        &(form.get("nimi").unwrap().to_string().replace("+", "_"))+"'</script></body></html>"
    });
    return creator;
}
fn vec_from_array<T>(array: Array<Option<T>>)->Vec<T>
{
    let mut vec: Vec<T> = Vec::new();
    for elem in array
    {
        vec.push(elem.unwrap());
    }
    return vec;
}
pub fn watcher()->Reititin
{
    let mut watcher: Reititin = Nickel::router();
    watcher.get("/event/:event_name", middleware!{| req, mut res| <ServerData>
        let conn = req.db_conn();
        let stmt = conn.prepare("SELECT * FROM tapahtuma WHERE nimi = $1").unwrap();
        let empty = String::new();
        let stmt_id = conn.prepare("SELECT id FROM kayttaja WHERE kayttajanimi = $1").unwrap();
        let mut guy = String::new();
        assert_eq!(guy, String::new());
        {guy= match *CookieSession::get_mut(req, &mut res)
            {
                Some(ref name)=>name,
                _ => &empty
            
            }.to_string();
        }
        let mut id: i32 = 0;
        for row in stmt_id.query(&[&guy]).unwrap()
        {
            id = row.get(0);
        }
        if id == 0
        {
            let mut data = HashMap::new();
            data.insert("Kirjaudu".to_string(), "Kirjaudu ulos".to_string());
            for row in stmt.query(&[&req.param("event_name").unwrap()]).unwrap()
            {
                data.insert("nimi".to_string(), parse(row.get(1)));
                data.insert("kuvaus".to_string(), parse(row.get(2)));
                let mut vartio_temp = "<ul>".to_string();
                let mut yleis_temp = "<ul>".to_string();
                let vartio_vec: Vec<String> = vec_from_array(row.get(6));
                let yleis_vec: Vec<String> = vec_from_array(row.get(3));
                for string in vartio_vec
                {
                    if string != "" || string != String::new()
                    {
                        vartio_temp = vartio_temp+"<li>"+&(string.replace("%0D%0A", ""))+
                            "</li>";
                    }
                }
                for string in yleis_vec
                {
                    if string != "" || string != String::new()
                    {
                        yleis_temp = yleis_temp+"<li>"+&(string.replace("%0D%0A", ""))+"</li>";
                    }
                }
                vartio_temp = vartio_temp+"</ul>";
                yleis_temp = yleis_temp+"</ul>";
                data.insert("vartiolista".to_string(), vartio_temp.replace("+", " "));
                data.insert("yleislista".to_string(), yleis_temp.replace("+", " "));
            }
            return res.render("assets/event.html", &data);
        } 
        else
        {
            print!("");        
            let mut data = HashMap::new();
            data.insert("Kirjaudu".to_string(), "Kirjaudu ulos".to_string());
            for row in stmt.query(&[&req.param("event_name").unwrap()]).unwrap()
            {
                data.insert("nimi".to_string(), parse(row.get(1)));
                data.insert("kuvaus".to_string(), parse(row.get(2)));
                let mut vartio_temp = "<form role='form'><ul>".to_string();
                let mut yleis_temp = "<ul>".to_string();
                let vartio_vec: Vec<String> = vec_from_array(row.get(6));
                let yleis_vec: Vec<String> = vec_from_array(row.get(3));
                for string in vartio_vec
                {
                    if string != "" || string != String::new()
                    {
                        vartio_temp = vartio_temp+"<li>"+&(string.replace("%0D%0A", ""))+
                            "</li><label><input type='checkbox' name='"+&string+
                            "' value='V'>Varaa</input></label>";
                    }
                }
                for string in yleis_vec
                {
                    if string != "" || string != String::new()
                    {
                        yleis_temp = yleis_temp+"<li>"+&(string.replace("%0D%0A", ""))+"</li>";
                    }
                }
                yleis_temp = yleis_temp+"</ul>";
                vartio_temp = vartio_temp+"</ul><button type='submit' 
                    formenctype='application/x-www-form-urlencoded' class='btn btn-default' formaction='/event/"+&req.param("event_name").unwrap()+"' method='post'>Varaa varusteet</form>";
                data.insert("vartiolista".to_string(), vartio_temp.replace("+", " "));
                data.insert("yleislista".to_string(), yleis_temp.replace("+", " "));
            }
            return res.render("assets/event.html", &data);
        }
    });
    return watcher;
}
