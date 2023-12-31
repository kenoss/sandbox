#![allow(unused)]

const EPSILON: f64 = 0.00000001;

#[derive(Debug, Clone, PartialEq)]
struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, PartialEq)]
struct Vector {
    pub x: f64,
    pub y: f64,
}

impl Vector {
    /// p1 - p2
    pub fn points_minus(p1: &Point, p2: &Point) -> Self {
        Self {
            x: p1.x - p2.x,
            y: p1.y - p2.y,
        }
    }

    pub fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalized(&self) -> Self {
        let norm = self.norm();
        Self {
            x: self.x / norm,
            y: self.y / norm,
        }
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y
    }

    pub fn cross(&self, other: &Self) -> f64 {
        self.y * other.x - self.x * other.y
    }

    /// 90度 rotate した単位法線ベクトル
    pub fn normalized_rot90(&self) -> Self {
        // Multiply
        //
        //   [[cos 90, -sin 90]
        //    [sin 90,  cos 90]]
        let norm = self.norm();
        Self {
            x: -self.y / norm,
            y: self.x / norm,
        }
    }
}

/// Check whether a polygon does contains a point.
fn polygon_contains_point(polygon: Vec<Point>, point: &Point) -> bool {
    assert!(polygon.len() >= 3);

    // まず「与えられた頂点を左回りに番号を振り直したときの内部」から取った点に対しての ∫arg(z) を計算して向きを判断し,
    // ∫arg(z) = 2π ケースに帰着.
    let mut ps = if integral_argz(&polygon) > 0.0 {
        polygon
    } else {
        polygon.into_iter().rev().collect()
    };

    // 三角形分割して各々判定
    // へこみになっていない ps[i], ps[i+1], ps[i+2] で囲まれる三角形を処理して n を下げていく.
    'loop_ps: while ps.len() > 3 {
        let n = ps.len();

        // ∫arg(z) = 2π なので全て凹みではない. 凸なやつを探す.
        let mut i = usize::MAX;
        for j in 0..n {
            let v = Vector::points_minus(&ps[(j + 2) % n], &ps[j]);
            let x = v
                .normalized_rot90()
                .dot(&Vector::points_minus(&ps[(j + 2) % n], &ps[(j + 1) % n]));
            let is_in_right = if x.abs() < EPSILON {
                ps.remove((j + 1) % n);
                continue 'loop_ps;
            } else {
                x > 0.0
            };

            if is_in_right {
                i = j;
                break;
            }
        }

        // ps[i+1] は外側にある.
        if triangle_contains_point(&ps[i], &ps[(i + 1) % n], &ps[(i + 2) % n], point) {
            return true;
        } else {
            ps.remove((i + 1) % n);
            continue;
        }
    }

    triangle_contains_point(&ps[0], &ps[1], &ps[2], point)
}

// Calculate ∫arg(z) of the closed path.
fn integral_argz(ps: &[Point]) -> f64 {
    let n = ps.len();
    let mut vs = vec![];
    for i in 0..(n - 1) {
        vs.push(Vector::points_minus(&ps[i + 1], &ps[i]).normalized());
    }
    vs.push(Vector::points_minus(&ps[0], &ps[n - 1]).normalized());
    vs.push(vs[0].clone());

    let mut argz: f64 = 0.0;
    for i in 0..n {
        let a = &vs[i];
        let b = &vs[i + 1];
        let cos = b.dot(a);
        let sin = b.cross(a);
        if sin.abs() < EPSILON && cos >= 0.0 {
            // argz += 0.0;
        } else if sin.abs() < EPSILON && cos < 0.0 {
            // めんどいので panic.
            // もしこういうのをサポートしたいのであれば, こういう点をスキップすると上手くいくはず.
            // なぜなら区分的に滑らかな自己交差のない閉曲線の連続変形で ∫arg(z) は定数なので.
            dbg!(a, b, cos, sin);
            panic!();
        } else if sin.is_sign_positive() {
            argz += cos.acos();
        } else {
            argz -= cos.acos();
        }
    }

    argz
}

// Assume that path p0 -> p1 -> p2 -> p0 has ∫arg(z) = 2π.
fn triangle_contains_point(p0: &Point, p1: &Point, p2: &Point, point: &Point) -> bool {
    fn is_in_left(p0: &Point, p1: &Point, point: &Point) -> bool {
        let x = Vector::points_minus(p1, p0)
            .normalized_rot90()
            .dot(&Vector::points_minus(point, p0));
        if x.abs() < EPSILON {
            true
        } else {
            x > 0.0
        }
    }

    is_in_left(p0, p1, point) && is_in_left(p1, p2, point) && is_in_left(p2, p0, point)
}

#[cfg(test)]
mod tests {
    use super::*;

    impl From<(f64, f64)> for Point {
        fn from((x, y): (f64, f64)) -> Self {
            Self { x, y }
        }
    }

    impl From<(f64, f64)> for Vector {
        fn from((x, y): (f64, f64)) -> Self {
            Self { x, y }
        }
    }

    fn points(xs: &[(f64, f64)]) -> Vec<Point> {
        xs.into_iter().map(|x| (*x).into()).collect()
    }

    fn check(xs: &[(f64, f64)], point: (f64, f64)) -> bool {
        let xs = xs.into_iter().map(|x| (*x).into()).collect::<Vec<Point>>();
        let point = point.into();
        polygon_contains_point(xs, &point)
    }

    #[test]
    fn test_vector_points_minus() {
        assert_eq!(
            Vector::points_minus(&(1.0, 0.0).into(), &(0.0, 1.0).into()),
            (1.0, -1.0).into()
        );
    }

    #[test]
    fn test_vector_normalized() {
        // Norm \sqrt 5 / 2
        assert_eq!(
            Vector::from((1.0, 2.0)).normalized(),
            Vector::from((0.4472135954999579, 0.8944271909999159))
        );
    }

    #[test]
    fn test_vector_cross() {
        // a = (1, 0)
        // b = (0, 1)
        // b x a = sin 90 = 1
        let a = Vector::from((1.0, 0.0));
        let b = Vector::from((0.0, 1.0));
        assert_eq!(b.cross(&a), 1.0);
    }

    #[test]
    fn test_integral_argz() {
        // 2π
        assert_eq!(
            integral_argz(&points(&[(0.0, 0.0), (1.0, 0.0), (0.0, 1.0)])),
            6.283185307179586
        );
        assert_eq!(
            integral_argz(&points(&[(0.0, 0.0), (1.0, 0.0), (0.0, 1.0), (-1.0, 2.0)])),
            6.283185307179585
        );
        assert_eq!(
            integral_argz(&points(&[
                (0.0, 0.0),
                (1.0, 0.0),
                (0.0, 1.0),
                (-1.0, 1.0),
                (0.5, -3.0)
            ])),
            6.283185307179587
        );
        // -2π
        assert_eq!(
            integral_argz(&points(&[(0.0, 0.0), (0.0, 1.0), (1.0, 0.0)])),
            -6.283185307179586
        );
        assert_eq!(
            integral_argz(&points(&[(0.0, 0.0), (-1.0, 2.0), (0.0, 1.0), (1.0, 0.0)])),
            -6.283185307179585
        );
        assert_eq!(
            integral_argz(&points(&[
                (0.0, 0.0),
                (0.5, -3.0),
                (-1.0, 2.0),
                (0.0, 1.0),
                (1.0, 0.0)
            ])),
            -6.283185307179585
        );
    }

    #[test]
    fn test_triangle() {
        // 2π
        assert!(check(&[(0.0, 0.0), (1.0, 0.0), (0.0, 1.0)], (0.0, 0.0)));
        assert!(!check(&[(0.0, 0.0), (1.0, 0.0), (0.0, 1.0)], (2.0, 0.0)));
        assert!(check(&[(0.0, 0.0), (1.0, 0.0), (0.0, 1.0)], (0.5, 0.49)));
        assert!(!check(&[(0.0, 0.0), (1.0, 0.0), (0.0, 1.0)], (0.5, 0.51)));
        assert!(check(&[(0.0, 0.0), (1.0, 0.0), (0.0, 1.0)], (0.01, 0.5)));
        assert!(!check(&[(0.0, 0.0), (1.0, 0.0), (0.0, 1.0)], (-0.01, 0.5)));
        // -2π
        assert!(check(&[(0.0, 0.0), (0.0, 1.0), (1.0, 0.0)], (0.0, 0.0)));
        assert!(!check(&[(0.0, 0.0), (0.0, 1.0), (1.0, 0.0)], (2.0, 0.0)));
        assert!(check(&[(0.0, 0.0), (0.0, 1.0), (1.0, 0.0)], (0.5, 0.49)));
        assert!(!check(&[(0.0, 0.0), (0.0, 1.0), (1.0, 0.0)], (0.5, 0.51)));
        assert!(check(&[(0.0, 0.0), (0.0, 1.0), (1.0, 0.0)], (0.01, 0.5)));
        assert!(!check(&[(0.0, 0.0), (0.0, 1.0), (1.0, 0.0)], (-0.01, 0.5)));
    }

    #[test]
    fn test_square() {
        // 2π
        assert!(check(
            &[(0.0, 0.0), (1.0, 0.0), (0.0, 1.0), (-1.0, 2.0)],
            (0.0, 0.0)
        ));
        assert!(!check(
            &[(0.0, 0.0), (1.0, 0.0), (0.0, 1.0), (-1.0, 2.0)],
            (2.0, 0.0)
        ));
        assert!(check(
            &[(0.0, 0.0), (1.0, 0.0), (0.0, 1.0), (-1.0, 2.0)],
            (0.5, 0.49)
        ));
        assert!(!check(
            &[(0.0, 0.0), (1.0, 0.0), (0.0, 1.0), (-1.0, 2.0)],
            (0.5, 0.51)
        ));
        assert!(check(
            &[(0.0, 0.0), (1.0, 0.0), (0.0, 1.0), (-1.0, 2.0)],
            (0.01, 0.5)
        ));
        assert!(check(
            &[(0.0, 0.0), (1.0, 0.0), (0.0, 1.0), (-1.0, 2.0)],
            (-0.01, 0.5)
        ));
        assert!(!check(
            &[(0.0, 0.0), (1.0, 0.0), (0.0, 1.0), (-1.0, 2.0)],
            (-0.5, 1.51)
        ));
        assert!(check(
            &[(0.0, 0.0), (1.0, 0.0), (0.0, 1.0), (-1.0, 2.0)],
            (-0.5, 1.49)
        ));
        assert!(check(
            &[(0.0, 0.0), (1.0, 0.0), (0.0, 1.0), (-1.0, 2.0)],
            (-0.5, 1.01)
        ));
        assert!(!check(
            &[(0.0, 0.0), (1.0, 0.0), (0.0, 1.0), (-1.0, 2.0)],
            (-0.5, 0.99)
        ));
        // -2π
        assert!(check(
            &[(0.0, 0.0), (-1.0, 2.0), (0.0, 1.0), (1.0, 0.0)],
            (0.0, 0.0)
        ));
        assert!(!check(
            &[(0.0, 0.0), (-1.0, 2.0), (0.0, 1.0), (1.0, 0.0)],
            (2.0, 0.0)
        ));
        assert!(check(
            &[(0.0, 0.0), (-1.0, 2.0), (0.0, 1.0), (1.0, 0.0)],
            (0.5, 0.49)
        ));
        assert!(!check(
            &[(0.0, 0.0), (-1.0, 2.0), (0.0, 1.0), (1.0, 0.0)],
            (0.5, 0.51)
        ));
        assert!(check(
            &[(0.0, 0.0), (-1.0, 2.0), (0.0, 1.0), (1.0, 0.0)],
            (0.01, 0.5)
        ));
        assert!(check(
            &[(0.0, 0.0), (-1.0, 2.0), (0.0, 1.0), (1.0, 0.0)],
            (-0.01, 0.5)
        ));
        assert!(!check(
            &[(0.0, 0.0), (-1.0, 2.0), (0.0, 1.0), (1.0, 0.0)],
            (-0.5, 1.51)
        ));
        assert!(check(
            &[(0.0, 0.0), (-1.0, 2.0), (0.0, 1.0), (1.0, 0.0)],
            (-0.5, 1.49)
        ));
        assert!(check(
            &[(0.0, 0.0), (-1.0, 2.0), (0.0, 1.0), (1.0, 0.0)],
            (-0.5, 1.01)
        ));
        assert!(!check(
            &[(0.0, 0.0), (-1.0, 2.0), (0.0, 1.0), (1.0, 0.0)],
            (-0.5, 0.99)
        ));
    }
}
