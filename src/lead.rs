use regex::Regex;

pub struct Lead {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub list: String,
}

#[allow(non_snake_case)]
impl Lead {
    pub fn extLead(cardDesc: &str, listName: &str) -> Lead {
        let lines: Vec<&str> = cardDesc.lines().collect();

        let name = lines[4].replace("Nome: ", "");
        
        let emailRgx = Regex::new(r"Email: \[.*?\]\(mailto:(.*?)\)").unwrap();
        let email = match emailRgx.captures(lines[5]) {
            Some(cap) => {
                if let Some(emailGroup) = cap.get(1) {
                    let mut email = emailGroup.as_str().to_string();

                    email = email.replace("\\[.*?\\]\\(mailto:|\"\\u200c\"\\)", "");
                    email = email.replace("[^\\(]*\\(mailto:|\"\\u200c\"\\)", "");
                    email = email.replace("\\)", "");
                    email = email.replace(" \"‌\"", "");

                    email.to_string()
                } else {
                    String::new()
                }
            }
            None => lines[5]
                .replace("Email: ", "")
        };
        
        let phone = lines[6].replace("Telefone: ", "");

        Lead { name, email, phone, list: listName.to_string() }
    }
}
