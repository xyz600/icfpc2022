use crate::{
    problem::{Color64, Color8},
    random::CachedRandom,
};

/// (assign table, color)
pub fn k_means_clustering(data: &Vec<Color8>, k: usize, random: &mut CachedRandom) -> (Vec<usize>, Vec<Color8>) {
    let mut assign_table = vec![0; data.len()];
    let mut center = vec![Color8::new(0, 0, 0, 0); k];
    let mut cluster_size = vec![0; k];

    // ランダム割り当て
    for i in 0..data.len() {
        let cluster_id = random.next_int_range(0, k as u32 - 1) as usize;
        assign_table[i] = cluster_id;
        cluster_size[cluster_id] += 1;
    }

    for _iter in 0..100 {
        // 中心の更新
        let mut temporal_center = vec![Color64::new(0.0, 0.0, 0.0, 0.0); k];
        for (idx, color) in data.iter().enumerate() {
            let assign = assign_table[idx];
            temporal_center[assign] += color.to64();
        }
        for idx in 0..k {
            temporal_center[idx] /= cluster_size[idx] as f64;
            center[idx] = temporal_center[idx].to8();
        }

        // 割り当ての変更
        cluster_size.fill(0);
        for (di, color) in data.iter().enumerate() {
            let mut best_assign = 0;
            let mut best_score = std::f64::MAX;
            for ci in 0..k {
                let score = (center[ci].to64() - color.to64()).square().horizontal_add();
                if score < best_score {
                    best_score = score;
                    best_assign = ci;
                }
            }
            assign_table[di] = best_assign;
            cluster_size[best_assign] += 1;
        }
    }

    (assign_table, center)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clustering() {
        let colors = vec![
            Color8::new(2, 2, 0, 0),
            Color8::new(0, 0, 2, 2),
            Color8::new(2, 0, 2, 0),
            Color8::new(0, 2, 0, 2),
            Color8::new(102, 102, 100, 100),
            Color8::new(100, 100, 102, 102),
            Color8::new(102, 100, 102, 100),
            Color8::new(100, 102, 100, 102),
        ];
        let mut random = CachedRandom::new(65536, 42);
        let (assign_table, color) = k_means_clustering(&colors, 2, &mut random);

        assert_eq!(assign_table[0], assign_table[1]);
        assert_eq!(assign_table[0], assign_table[2]);
        assert_eq!(assign_table[0], assign_table[3]);
        assert_eq!(assign_table[4], assign_table[5]);
        assert_eq!(assign_table[4], assign_table[6]);
        assert_eq!(assign_table[4], assign_table[7]);
        assert_ne!(assign_table[0], assign_table[4]);

        let color_min = if color[0].r < color[1].r { color[0] } else { color[1] };
        let color_max = if color[0].r < color[1].r { color[1] } else { color[0] };

        assert_eq!(color_min.r, 1);
        assert_eq!(color_max.r, 101);
    }
}
