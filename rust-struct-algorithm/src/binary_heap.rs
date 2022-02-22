macro_rules! parent {
    ($child:ident) => {
        $child >> 1
    };
}

macro_rules! left_child {
    ($parent:ident) => {
        $parent << 1
    };
}

macro_rules! right_child {
    ($parent:ident) => {
        ($parent << 1) + 1
    };
}
#[derive(Debug, Clone)]
struct BinaryHeap {
    size: usize,
    data: Vec<i32>,
}

impl BinaryHeap {
    fn new() -> Self {
        BinaryHeap {
            size: 0,
            data: vec![0],
        }
    }

    fn len(&self) -> usize {
        self.size
    }

    fn is_empty(&self) -> bool {
        0 == self.size
    }

    fn min(&self) -> Option<i32> {
        if self.size == 0 {
            None
        } else {
            Some(self.data[1])
        }
    }
}

impl BinaryHeap {
    fn push(&mut self, val: i32) {
        self.data.push(val);
        self.size += 1;
        self.move_up(self.size);
    }

    fn move_up(&mut self, mut c: usize) {
        loop {
            let p = parent!(c);
            if p <= 0 {
                break;
            }
            if self.data[c] < self.data[p] {
                self.data.swap(c, p);
            }
            c = p;
        }
    }

    fn pop(&mut self) -> Option<i32> {
        if 0 == self.size {
            None
        } else if 1 == self.size {
            self.size -= 1;
            self.data.pop()
        } else {
            self.data.swap(1, self.size);
            let val = self.data.pop();
            self.size -= 1;
            self.move_down(1);
            val
        }
    }

    fn move_down(&mut self, mut c: usize) {
        loop {
            let lc = left_child!(c);
            if lc > self.size {
                break;
            }
            let mc = self.min_child(c);
            if self.data[c] > self.data[mc] {
                self.data.swap(c, mc);
            }
            c = mc
        }
    }

    // 最小子节点位置
    fn min_child(&self, i: usize) -> usize {
        let (lc, rc) = (left_child!(i), right_child!(i));
        if rc > self.size {
            lc
        } else if self.data[lc] < self.data[rc] {
            lc
        } else {
            rc
        }
    }
}

impl BinaryHeap {
    // 构建新堆

    fn build_new(&mut self, arr: &[i32]) {
        for _i in 0..self.size {
            let _rm = self.data.pop();
        }

        for &val in arr {
            self.data.push(val)
        }
        self.size = arr.len();

        // 调整小顶堆
        let size = self.size;
        let mut p = parent!(size);
        while p > 0 {
            self.move_down(p);
            p -= 1;
        }
    }

    fn build_add(&mut self, arr: &[i32]) {
        for &val in arr {
            self.push(val);
        }
    }
}
