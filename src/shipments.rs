use rand::Rng;

// Function to count the minimum number of moves to balance the shipments
fn count_permutation(shipments: &Vec<u32>) -> usize {
    let total: u32 = shipments.iter().sum();
    let n = shipments.len() as u32;

    // Check if an even distribution is possible
    if total % n != 0 {
        return usize::MAX; // Return max value if not possible
    }

    let average = total / n;
    let mut excess_moves = 0;
    let mut deficit_moves = 0;

    for &shipment in shipments {
        if shipment > average {
            excess_moves += shipment - average;
        } else {
            deficit_moves += average - shipment;
        }
    }

    // The total moves required to balance will be the maximum of excess or deficit moves
    excess_moves.max(deficit_moves) as usize
}

// Function to check if even distribution is possible
fn can_distribute_evenly(shipments: &Vec<u32>) -> bool {
    let total: u32 = shipments.iter().sum();
    let n = shipments.len() as u32;

    total % n == 0
}

// Function to generate a vector with randomly distributed shipments
fn gen_shipments(n: usize) -> Vec<u32> {
    let average = 100; // Average value for generation
    let mut shipments = vec![average; n];

    // Decrease random elements to create variety
    let mut rng = rand::thread_rng();
    for i in 0..n {
        shipments[i] = average - (rng.gen_range(0..20) % 10) as u32; // Generate values from 0 to 10
    }

    shipments
}

// Example usage
pub fn main() {
    let shipments1 = vec![8, 2, 2, 4, 4];
    let shipments2 = vec![9, 3, 7, 2, 9];

    // First example
    let moves1 = count_permutation(&shipments1);
    let can_distribute1 = can_distribute_evenly(&shipments1);
    println!("Moves needed for {:?}: {}, Can distribute evenly: {}", shipments1, moves1, can_distribute1);

    // Second example
    let moves2 = count_permutation(&shipments2);
    let can_distribute2 = can_distribute_evenly(&shipments2);
    println!("Moves needed for {:?}: {}, Can distribute evenly: {}", shipments2, moves2, can_distribute2);
}
