pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    // todo!("based on the {diagram}, determine the plants the {student} is responsible for");
    let mut v: Vec<&'static str> = Vec::new();
    let rows = diagram.split("\n").collect::<Vec<&str>>();
    let l0 = student.chars().next().unwrap();
    let offset = (l0 as u8 - 'A' as u8) as usize * 2;
    let r1 = &(rows[0])[offset..offset+2];
    let r2 = &(rows[1])[offset..offset+2];
    let iter = r1.chars().chain(r2.chars());
    iter.for_each(|c| {
        match c {
            'V' => v.push("violets"),
            'G' => v.push("grass"),
            'C' => v.push("clover"),
            'R' => v.push("radishes"),
            _ => {}
        };
    });
    v
}
