extern crate csv;
extern crate serde_json;

use std::io;
use std::process;
use serde_json::value::Value;

pub fn json_to_csv(stdin: &str, csv_delimiter: u8) -> io::Result<()> {
    let mut is_array_of_objects = false;
    let mut wtr = csv::WriterBuilder::new()
        .delimiter(csv_delimiter)
        .from_writer(io::stdout());
    if let Ok(json) = serde_json::from_str::<Value>(stdin) {
        if let Some(entries) = json.as_array() {
            if let Some(example) = entries.first() {
                if let Some(example_object) = example.as_object() {
                    let ks = example_object.keys();
                    // WRITE THE HEADER using example
                    wtr.write_record(ks)?;
                    wtr.flush()?;
                    is_array_of_objects = true;
                } else {
                    eprintln!("Expected a JSON array of objects. Got: {}", json);
                    process::exit(1);
                }
            } else {
                eprintln!("Will not write empty CSV, got empty JSON array.");
                process::exit(1);
            }

            // RECORDS
            if is_array_of_objects {
                for entry in entries {
                    let o = entry.as_object().unwrap();
                    let vals = o.values().map(|x: &Value| match *x {
                        Value::Null => "null".to_owned(),
                        Value::Bool(ref b) => format!("{}", b),
                        Value::Number(ref n) => format!("{}", n),
                        Value::String(ref s) => s.to_owned(),
                        Value::Array(ref a) => {
                            eprintln!("Nested arrays not supported. Found {:?}", a);
                            process::exit(1);
                        }
                        Value::Object(ref o) => {
                            eprintln!("Nested objects not supported. Found {:?}", o);
                            process::exit(1);
                        }
                    });
                    wtr.write_record(vals)?;
                }
            }
        } else {
            eprintln!("Expected a JSON array of objects. Got: {}", json);
            process::exit(1);
        }
    } else {
        eprintln!("Non-JSON found: {}", stdin);
        process::exit(1);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
