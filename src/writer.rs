use csv::WriterBuilder;
use crate::lead::Lead;
use std::env;

pub struct Writer;

#[allow(non_snake_case)]
impl Writer {
    pub fn saveLeads(leads: &Vec<Lead>, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        if !leads.is_empty() {
            let path = match env::var("HOME") {
                Ok(home) => format!("{}/Desktop", home),
                Err(_) => String::from("Desktop"),
            };

            let filepath = format!("{}/{}", path, filename);

            match WriterBuilder::new().from_path(&filepath) {
                Ok(mut writer) => {
                    writer.write_record(&["List from", "Name", "Email", "Phone"]).unwrap();

                    for lead in leads.iter() {
                        let _ = writer.write_record(&[&lead.list, &lead.name, &lead.email, &lead.phone]);
                    }

                    Ok(())
                }
                Err(e) => Err(e.into()),
            }
        } else {
            Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "No leads to save.",
            )))
        }
    }
}
