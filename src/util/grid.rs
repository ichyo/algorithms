static DY: [isize; 8] = [0, 1, 0, -1, 1, -1, 1, -1];
static DX: [isize; 8] = [1, 0, -1, 0, 1, 1, -1, -1];

fn try_adj(y: usize, x: usize, dy: isize, dx: isize, h: usize, w: usize) -> Option<(usize, usize)> {
    let ny = y as isize + dy;
    let nx = x as isize + dx;
    if ny >= 0 && nx >= 0 {
        let ny = ny as usize;
        let nx = nx as usize;
        if ny < h && nx < w {
            Some((ny, nx))
        } else {
            None
        }
    } else {
        None
    }
}

pub struct Adj4 {
    y: usize,
    x: usize,
    h: usize,
    w: usize,
    r: usize,
}

impl Iterator for Adj4 {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.r >= 4 {
                return None;
            }

            let dy = DY[self.r];
            let dx = DX[self.r];
            self.r += 1;

            if let Some((ny, nx)) = try_adj(self.y, self.x, dy, dx, self.h, self.w) {
                return Some((ny, nx));
            }
        }
    }
}

pub fn adj4_iter(y: usize, x: usize, h: usize, w: usize) -> Adj4 {
    Adj4 { y, x, h, w, r: 0 }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_adj4_normal() {
        let mut iter = adj4_iter(2, 1, 4, 4);
        assert_eq!(Some((2, 2)), iter.next());
        assert_eq!(Some((3, 1)), iter.next());
        assert_eq!(Some((2, 0)), iter.next());
        assert_eq!(Some((1, 1)), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn test_adj4_upperleft() {
        let mut iter = adj4_iter(0, 0, 4, 4);
        assert_eq!(Some((0, 1)), iter.next());
        assert_eq!(Some((1, 0)), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn test_adj4_lowerright() {
        let mut iter = adj4_iter(3, 3, 4, 4);
        assert_eq!(Some((3, 2)), iter.next());
        assert_eq!(Some((2, 3)), iter.next());
        assert_eq!(None, iter.next());
    }
}
