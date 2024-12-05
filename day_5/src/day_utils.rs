pub fn parse_input(input: &str) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let mut sections = input.split("\n\n");
    let rules_section = sections.next().unwrap();
    let updates_section = sections.next().unwrap();

    let rules = rules_section
        .lines()
        .map(|line| {
            let parts: Vec<i32> = line.split('|').map(|x| x.parse().unwrap()).collect();
            (parts[0], parts[1])
        })
        .collect();

    let updates = updates_section
        .lines()
        .map(|line| {
            line.split(',')
                .map(|x| x.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    (rules, updates)
}
