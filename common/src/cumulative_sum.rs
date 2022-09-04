use std::ops::{Add, AddAssign, Sub, SubAssign};

use crate::problem::{Color64, Color8, Image};

/// u8 の 2次元バッファに対して median を高速に求める
pub struct RangeMedianCalculator {
    buffer: Vec<CumulativeSum<usize>>,
}

impl RangeMedianCalculator {
    pub fn new(buffer: &Vec<Vec<u8>>) -> RangeMedianCalculator {
        let height = buffer.len();
        let width = buffer[0].len();

        let mut count_table = vec![vec![vec![0; width]; height]; 256];

        for y in 0..height {
            for x in 0..width {
                let val = buffer[y][x] as usize;
                count_table[val][y][x] = 1;
            }
        }

        let mut buffer = vec![];
        for val in 0..256 {
            buffer.push(CumulativeSum::<usize>::new(&count_table[val]));
        }

        RangeMedianCalculator { buffer }
    }

    pub fn median(&self, sy: usize, sx: usize, ey: usize, ex: usize) -> u8 {
        let threashold = (1 + (ey - sy) * (ex - sx)) / 2;
        let mut sum = 0;
        for val in 0..256 {
            let freq = self.buffer[val].range_sum(sy, sx, ey, ex);
            if sum < threashold && threashold <= sum + freq {
                return val as u8;
            }
            sum += freq;
        }
        255
    }
}

pub struct RangeColorMedianCalculator {
    range_median_calculator: [RangeMedianCalculator; 4],
}

impl RangeColorMedianCalculator {
    pub fn new(image: &Image) -> RangeColorMedianCalculator {
        let height = image.height;
        let width = image.width;

        let mut buffer = [
            vec![vec![0; width]; height],
            vec![vec![0; width]; height],
            vec![vec![0; width]; height],
            vec![vec![0; width]; height],
        ];
        for y in 0..height {
            for x in 0..width {
                let color = image.color_of(y, x);
                buffer[0][y][x] = color.r;
                buffer[1][y][x] = color.g;
                buffer[2][y][x] = color.b;
                buffer[3][y][x] = color.a;
            }
        }

        let r_median = RangeMedianCalculator::new(&buffer[0]);
        let g_median = RangeMedianCalculator::new(&buffer[1]);
        let b_median = RangeMedianCalculator::new(&buffer[2]);
        let a_median = RangeMedianCalculator::new(&buffer[3]);

        RangeColorMedianCalculator {
            range_median_calculator: [r_median, g_median, b_median, a_median],
        }
    }

    pub fn median(&self, sy: usize, sx: usize, ey: usize, ex: usize) -> Color8 {
        let median_r = self.range_median_calculator[0].median(sy, sx, ey, ex);
        let median_g = self.range_median_calculator[1].median(sy, sx, ey, ex);
        let median_b = self.range_median_calculator[2].median(sy, sx, ey, ex);
        let median_a = self.range_median_calculator[3].median(sy, sx, ey, ex);

        Color8::new(median_r, median_g, median_b, median_a)
    }
}

pub struct CumulativeRMSESum {
    mean_sum: CumulativeSum<Color64>,
    squared_mean_sum: CumulativeSum<Color64>,
}

impl CumulativeRMSESum {
    pub fn new(image: &Image) -> CumulativeRMSESum {
        let height = image.height;
        let width = image.width;

        let mut data = vec![vec![Color64::default(); width]; height];
        let mut squared_data = vec![vec![Color64::default(); width]; height];

        for y in 0..height {
            for x in 0..width {
                let target_color = image.color_of(y, x).to64();

                data[y][x] = target_color;
                squared_data[y][x] = target_color.square();
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
        t1 - t2
    }

    // FIXME:
    pub fn mean_color(&self, sy: usize, sx: usize, ey: usize, ex: usize) -> Color8 {
        let size = (ey - sy) * (ex - sx);
        (self.mean_sum.range_sum(sy, sx, ey, ex) / (size as f64)).round().to8()
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
    T: Add<T, Output = T> + AddAssign<T> + Sub<T, Output = T> + SubAssign<T> + Default + Clone + Copy,
{
    pub fn new(data: &Vec<Vec<T>>) -> CumulativeSum<T> {
        let height = data.len() + 1;
        let width = data[0].len() + 1;
        let mut ret_data = vec![vec![T::default(); width]; height];

        for y in 0..height - 1 {
            for x in 0..width - 1 {
                ret_data[y + 1][x + 1] = data[y][x] + ret_data[y][x + 1] + ret_data[y + 1][x] - ret_data[y][x];
            }
        }

        CumulativeSum { cumulative_data: ret_data }
    }

    pub fn range_sum(&self, sy: usize, sx: usize, ey: usize, ex: usize) -> T {
        self.cumulative_data[ey][ex] + self.cumulative_data[sy][sx] - self.cumulative_data[ey][sx] - self.cumulative_data[sy][ex]
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

        let ret = cumulator.range_sum(1, 1, 4, 4);
        assert_eq!(ret, 36);

        let ret = cumulator.range_sum(1, 2, 3, 4);
        assert_eq!(ret, 16);
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

        let rmse_calculator = CumulativeRMSESum::new(&image);
        let ret = rmse_calculator.range_rmse(1, 1, 3, 3);

        let mean = (2.0 + 3.0 + 3.0 + 4.0) / 4.0f64;
        let val = (2.0 - mean).powi(2) + (3.0 - mean).powi(2) * 2.0 + (4.0 - mean).powi(2);
        let expected = Color64::new(val, val, val, val);
        assert!((ret - expected).horizontal_add() < 1e-7);
    }

    #[test]
    fn test_median() {
        let data = vec![
            vec![0, 1, 1, 2, 2, 3, 3],
            vec![1, 2, 2, 3, 3, 4, 4],
            vec![0, 1, 1, 2, 2, 3, 3],
            vec![0, 1, 1, 2, 2, 3, 3],
            vec![0, 2, 2, 2, 2, 3, 3],
            vec![0, 1, 1, 2, 2, 3, 3],
            vec![0, 1, 1, 2, 2, 3, 3],
        ];

        let median_calculator = RangeMedianCalculator::new(&data);
        let med = median_calculator.median(1, 1, 4, 4);
        assert_eq!(med, 2);
    }
}
