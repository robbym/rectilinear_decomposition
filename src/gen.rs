use ::rand::rngs::StdRng;
use ::rand::seq::IteratorRandom;
use ::rand::SeedableRng;

pub fn generate_random_horizontally_convex_polygon(
    seed: [u8; 32],
    height: usize,
    min_v_corridor: i64,
) -> Vec<(i64, i64)> {
    fn create_chain(seed: [u8; 32], height: usize) -> (Vec<(i64, i64)>, i64, i64) {
        let mut rng = StdRng::from_seed(seed);
        let mut chain: Vec<(i64, i64)> = Vec::new();
        chain.push((0, 0));
        chain.push((0, 1));

        let mut last_direction: (i64, i64) = (0, 1);

        let mut min_x = 0;
        let mut max_x = 0;

        loop {
            let (lx, ly) = chain[chain.len() - 1];

            if lx < min_x {
                min_x = lx;
            }
            if lx > max_x {
                max_x = lx;
            }

            if ly >= height as i64 {
                break;
            }

            let direction = [(0, 1), (-1, 0), (1, 0)]
                .into_iter()
                .filter(|(px, _)| (px - last_direction.0).abs() != 2)
                .choose(&mut rng)
                .unwrap();

            match direction {
                (0, 1) => {
                    if last_direction == (0, 1) {
                        chain.pop();
                    }
                    chain.push((lx, ly + 1));
                }
                (1, 0) => {
                    if last_direction != (0, 1) {
                        chain.pop();
                    }
                    chain.push((lx + 1, ly));
                }
                (-1, 0) => {
                    if last_direction != (0, 1) {
                        chain.pop();
                    }
                    chain.push((lx - 1, ly));
                }
                _ => {}
            }

            last_direction = direction;
        }

        (chain, min_x, max_x)
    }

    let (mut l_chain, _, l_max) = create_chain(seed, height);
    let (mut r_chain, r_min, _) = create_chain(seed, height);

    let needed_width = l_max - r_min + min_v_corridor;
    let left_offset = needed_width / 2;
    let right_offset = needed_width - left_offset;

    for (x, _) in &mut l_chain {
        *x -= left_offset;
    }

    for (x, _) in &mut r_chain {
        *x += right_offset;
    }

    l_chain.append(&mut r_chain);

    l_chain
}
