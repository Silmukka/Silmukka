///Luodaan parseri formille, jotta purkka ei näy muualla
use std::collections::HashMap;
pub fn handle_form(_form: &mut str)->HashMap<String, String>
{
    let mut form_data = HashMap::new();
    let mut form = _form.to_string();
    loop{
        if form.is_empty() == true{
            break;
        }
        let mut temp_hash = ("".to_string(), "".to_string());
        let mut temp = "".to_string();
        for c in form.clone().chars(){
            print!("{}", c);
            if c == '&'
            {
                temp_hash.1 = temp.clone();
                temp.remove(0);
                break;
            }
            if c == '='
            {
                temp_hash.0 = temp;
                temp = "".to_string();
                temp.remove(0);
            }
            else
            {
                temp.push(c);
                form.remove(0);
            }
        }
        form_data.insert(temp_hash.0, temp_hash.1);
        if form.is_empty() == true{
            break;
        }
        
    }
    println!("{:?}", form_data);
    return form_data;
 
}
