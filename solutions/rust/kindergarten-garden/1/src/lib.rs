pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    // todo!("based on the {diagram}, determine the plants the {student} is responsible for");
    // assignment: alphabetically, Alice...Larry
    // 4 cups, two per row
    let students = ["Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph", "Kincaid", "Larry"];
    let mut result = Vec::new();

    for (index, name) in students.iter().enumerate() {
        if *name == student {
            let start = index * 2;

            for row in diagram.lines() {
                for cup in row.chars().skip(start).take(2) {
                    let plant = match cup {
                        'G' => "grass",
                        'C' => "clover",
                        'R' => "radishes",
                        'V' => "violets",
                        _ => panic!("Unknown plant: {cup}"),
                    };
                    result.push(plant);
                }
            }
            break;
        }
    }

    result
}
