pub fn process_map(map: Vec<Vec<char>>) -> u32 {
    let mut count_infinite_loop: u32 = 0;
    let max_bounds = crate::day_utils::max_indices(&map).unwrap();

    let dot_positions: Vec<(usize, usize)> = map
        .iter()
        .enumerate()
        .flat_map(|(row_idx, row)| {
            row.iter()
                .enumerate()
                .filter(|&(_, &cell)| cell == '.')
                .map(move |(col_idx, _)| (row_idx, col_idx))
                .collect::<Vec<_>>()
        })
        .collect();

    for (row_idx, col_idx) in dot_positions {
        let mut current_map = map.clone();

        if current_map[row_idx][col_idx] == '.' {
            current_map[row_idx][col_idx] = '0';
        } else {
            break
        }
        let guard_index = crate::day_utils::find_char(&current_map, '^').unwrap();
        let mut guard = crate::first_part::Guard::new(guard_index, (-1, 0));
        while guard.move_forward(&current_map, max_bounds) {};

        if guard.infinite_loop {
            count_infinite_loop += 1;
        }
    }

    count_infinite_loop
}
