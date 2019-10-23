extern crate assert_approx_eq;


struct Graph<'a> {
    points: &'a [(f64, f64)]
}

impl<'a> Graph<'a> {
    fn new(points: &'a [(f64, f64)]) -> Self {
        Graph { points }
    }
    fn real_area(&self) -> f64 {
        fn min(a: f64, b: f64) -> f64 {
            if a - b <= 0_f64 { a } else { b }
        }

        let length = self.points.len();
        let mut area = 0_f64;
        for i in 1..length {
            let frame_area = min(self.points[i].1, self.points[i - 1].1)
                * (self.points[i].0 - self.points[i - 1].0);
            dbg!("{}",frame_area);
            area += frame_area;
        };
        area
    }

    fn monte_carlo_area(&self) ->f64{
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use crate::Graph;

    #[test]
    fn new_graph_test() {
        let vec: Vec<(f64, f64)> = (0..10).map(|x| (x as f64, 0.3)).collect();
        let g = Graph::new(&vec);
        let sum = g.points.iter().fold(0_f64, |accum, cur| { accum + cur.1 });
        assert_approx_eq::assert_approx_eq!(sum, 10_f64 * 0.3, 1.0e-6);
    }

    #[test]
    fn real_area_test() {
        let vec: Vec<(f64, f64)> = (0..10).map(|x| (x as f64, 0.3)).collect();
        let g = Graph::new(&vec);
        let real_area = g.real_area();
        assert_approx_eq::assert_approx_eq!(real_area, 0.3*((g.points.len()-1) as f64 ), 1.0e-6);
    }
}