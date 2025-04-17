use itertools::Itertools;

// ------------------------------------------------------------------------------------------------
// Exports

pub fn unique_key_lock_pairs_count(input: &str) -> usize {
    let (mut locks, mut keys) = (Vec::with_capacity(256), Vec::with_capacity(256));

    // Parse locks and keys schematics
    for line in input.split("\n\n") {
        let schematic = line.as_bytes().split(|&b| b == b'\n').collect_vec();
        let mut pin_heights: [u8; 5] = [0, 0, 0, 0, 0];
        let symbol = if schematic[0] == b"#####" { b'.' } else { b'#' };

        for row in &schematic[1..6] {
            for (i, &b) in row.iter().enumerate() {
                if b == symbol {
                    pin_heights[i] += 1;
                }
            }
        }

        if symbol == b'.' {
            locks.push(pin_heights);
        } else {
            keys.push(pin_heights);
        }
    }

    // Check all possible lock-key combinations;
    // the way parsing is done allows for quick comparison
    locks
        .into_iter()
        .cartesian_product(keys)
        .filter(|(lock, key)| lock.iter().zip(key.iter()).all(|(l, k)| l >= k))
        .count()
}
