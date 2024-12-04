fn main() {
    let penguien_data = "common name, length (cm)
    Little penguin, 33
    Yellow-eyed penguin, 65
    Fiordland penguin, 60
    Invalid, data
    ";

    let records = penguien_data.lines();

    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len()==0 {
            continue;
        }

        let fields: Vec<_> = record.split(',').collect();
        if cfg!(debug_assertions) {
            eprintln!("debug: {:?}, -> {:?}",
            record, fields);
        }

        if fields.len() >= 2 {
            let name = fields[0];
            if let Ok(length) = fields[1].trim().parse::<i32>() {
                println!("Penguin species: {}, length: {}cm", name, length);
            }
        }
    }
}
