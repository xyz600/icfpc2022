use std::ops::{Add, AddAssign, Sub, SubAssign};

use crate::problem::{Color64, Color8, Image, State};

pub struct CumulativeRMSESum {
    mean_sum: CumulativeSum<Color64>,
    squared_mean_sum: CumulativeSum<Color64>,
}

impl CumulativeRMSESum {
    pub fn new(image: &Image, state: &State) -> CumulativeRMSESum {
        let height = image.height;
        let width = image.width;

        let state_data = state.to_color_buffer();

        let mut data = vec![vec![Color64::default(); width]; height];
        let mut squared_data = vec![vec![Color64::default(); width]; height];

        for y in 0..height {
            for x in 0..width {
                let target_color = image.color_of(y, x).to64();
                let state_color = state_data[y][x].to64();
                let diff = target_color - state_color;
                data[y][x] = diff;
                squared_data[y][x] = diff.square();
            }
        }

        CumulativeRMSESum {
            mean_sum: CumulativeSum::new(&data),
            squared_mean_sum: CumulativeSum::new(&squared_data),
        }
    }

    /// 区間内の RMSE を求める
    /// [sx, ex), [sy, ey)
    pub fn range_rmse(&self, sy: usize, sx: usize, ey: usize, ex: usize) -> Color64 {
        let size = (ey - sy) * (ex - sx);
        let t1 = self.squared_mean_sum.range_sum(sy, sx, ey, ex);
        let t2 = self.mean_sum.range_sum(sy, sx, ey, ex).square() / (size as f64);
        (t1 - t2).sqrt()
    }
}

pub struct CumulativeSum<T>
where
    T: Add<T> + AddAssign<T> + Sub<T, Output = T> + SubAssign<T> + Default + Clone + Copy,
{
    cumulative_data: Vec<Vec<T>>,
}

impl<T> CumulativeSum<T>
where
    T: Add<T, Output = T>
        + AddAssign<T>
        + Sub<T, Output = T>
        + SubAssign<T>
        + Default
        + Clone
        + Copy,
{
    pub fn new(data: &Vec<Vec<T>>) -> CumulativeSum<T> {
        let height = data.len() + 1;
        let width = data[0].len() + 1;
        let mut ret_data = vec![vec![T::default(); width]; height];

        for y in 0..height - 1 {
            for x in 0..width - 1 {
                ret_data[y + 1][x + 1] =
                    data[y][x] + ret_data[y][x + 1] + ret_data[y + 1][x] - ret_data[y][x];
            }
        }

        CumulativeSum {
            cumulative_data: ret_data,
        }
    }

    pub fn range_sum(&self, sy: usize, sx: usize, ey: usize, ex: usize) -> T {
        self.cumulative_data[ey][ex] + self.cumulative_data[sy][sx]
            - self.cumulative_data[ey][sx]
            - self.cumulative_data[sy][ex]
    }

    pub fn height(&self) -> usize {
        self.cumulative_data.len()
    }

    pub fn width(&self) -> usize {
        self.cumulative_data[0].len()
    }
}

#[cfg(test)]
mod tests {
    use crate::problem::Command;

    use super::*;

    #[test]
    fn test_cumulate() {
        let data = vec![
            vec![0, 1, 2, 3, 4],
            vec![1, 2, 3, 4, 5],
            vec![2, 3, 4, 5, 6],
            vec![3, 4, 5, 6, 7],
            vec![4, 5, 6, 7, 8],
        ];
        let cumulator = CumulativeSum::new(&data);
        let ret = cumulator.range_sum(1, 1, 3, 3);
        assert_eq!(ret, 12);
    }

    #[test]
    fn test_rmse_compute() {
        let image = Image {
            height: 3,
            width: 3,
            buffer: vec![
                Color8::new(0, 0, 0, 0),
                Color8::new(1, 1, 1, 1),
                Color8::new(2, 2, 2, 2),
                Color8::new(1, 1, 1, 1),
                Color8::new(2, 2, 2, 2),
                Color8::new(3, 3, 3, 3),
                Color8::new(2, 2, 2, 2),
                Color8::new(3, 3, 3, 3),
                Color8::new(4, 4, 4, 4),
            ],
        };

        let mut state = State::new(3, 3);
        state.apply(Command::Color(0, Color8::new(0, 0, 0, 0)));
        let rmse_calculator = CumulativeRMSESum::new(&image, &state);
        let ret = rmse_calculator.range_rmse(1, 1, 3, 3);

        let mean = (2.0 + 3.0 + 3.0 + 4.0) / 4.0f64;
        let val = (2.0 - mean).powi(2) + (3.0 - mean).powi(2) * 2.0 + (4.0 - mean).powi(2);
        let expected = Color64::new(val, val, val, val).sqrt();
        assert!((ret - expected).horizontal_add() < 1e-7);
    }
}
