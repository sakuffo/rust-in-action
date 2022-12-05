fn main() { 
    // Create a CSV data structure
    let penguin_data = "\
    common name,length (cm)
    Little penguin,33
    Yellow-eyed penguin,65
    Fiordland penguin,60
    invalid,data
    ";

    // read in the lines from the CSV data into records
    let records = penguin_data.lines();

    println!("********************************");
    for (i, record) in records.enumerate(){
        if i == 0 || record.trim().len() == 0 {
            continue;
        }

        // split the record into fields
        let fields: Vec<_> = record
            .split(',')
            .map(|field| field.trim())
            .collect();
        if cfg!(debug_assertions) {
            eprintln!("debug: {:?} -> {:?}", 
                    record, fields);
        }

        let name = fields[0];
        if let Ok(length) = fields[1].parse::<f32>(){
            println!("{}, {}cm", name, length);
        }
    }
    println!("********************************");
}
