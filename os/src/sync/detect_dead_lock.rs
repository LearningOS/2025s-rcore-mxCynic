use core::fmt::Display;

use alloc::vec;
use alloc::vec::Vec;

/// data for dead lock detect
pub struct DeadLockDetector {
    avialiable: Vec<usize>,
    allocation: Vec<Vec<usize>>,
    need: Vec<Vec<usize>>,
}

impl Default for DeadLockDetector {
    fn default() -> Self {
        DeadLockDetector::new()
    }
}
impl DeadLockDetector {
    /// new a deadLock
    pub fn new() -> Self {
        Self {
            avialiable: Vec::new(),
            allocation: Vec::new(),
            need: Vec::new(),
        }
    }

    /// detect deac lock is safe or not safe
    /// true is safe false is not safe
    pub fn detect(&self) -> bool {
        let n = self.allocation.len();
        // let m = self.avialiable.len();

        let mut work = self.avialiable.clone();
        let mut finish = vec![false; n];

        loop {
            let mut found = false;

            for (i, if_finish) in finish.clone().iter().enumerate() {
                // step2: finish[i] 是假的 && 所有的need[i][j] <= work[j]
                if !if_finish
                    && self.need[i]
                        .iter()
                        .enumerate()
                        .all(|(j, &need)| need <= work[j])
                {
                    // setp3
                    for (j, &allocation) in self.allocation[i].iter().enumerate() {
                        work[j] += allocation;
                    }
                    finish[i] = true;

                    // 如果step2找到了一个可以运行的线程，
                    found = true;
                }
            }
            // 没有找到可以运行的线程，退出loop
            if !found {
                break;
            }
        }

        // step4: 全部是finish是true,否则不安全
        let res = finish.iter().all(|&x| x);
        res
    }

    /// init DeadLockDetector's shape
    pub fn init(&mut self, n: usize, m: usize, sem: usize) {
        //
        for _ in 0..m {
            self.avialiable.push(sem)
        }
        let vec = vec![0; m];

        for _ in 0..n {
            self.allocation.push(vec.clone());
            self.need.push(vec.clone());
        }
    }

    /// when create new mutex
    pub fn incre_m(&mut self, res_count: usize) {
        self.avialiable.push(res_count);

        for i in 0..self.need.len() {
            self.allocation[i].push(0);
            self.need[i].push(0);
        }
    }
    /// when new thread create
    pub fn incre_n(&mut self, tid: usize) {
        let resource_count = if self.allocation.is_empty() {
            0
        } else {
            self.allocation[0].len()
        };
        // 确保 allocation 和 need 至少有 tid+1 行
        while self.allocation.len() <= tid {
            self.allocation.push(vec![0; resource_count]);
            self.need.push(vec![0; resource_count]);
        }
    }

    /// when try lock 意味着需要一个资源
    pub fn need(&mut self, tid: usize, rid: usize) {
        self.need[tid][rid] += 1;
    }
    /// 检查到死锁，把死锁前尝试的值回复
    pub fn deneed(&mut self, tid: usize, rid: usize) {
        self.need[tid][rid] -= 1;
    }

    /// try lock
    pub fn allocate(&mut self, tid: usize, rid: usize) {
        self.avialiable[rid] -= 1;
        self.allocation[tid][rid] += 1;
        self.need[tid][rid] -= 1;
    }

    /// when unlock
    pub fn dealloc(&mut self, tid: usize, rid: usize) {
        self.avialiable[rid] += 1;
        self.allocation[tid][rid] -= 1;
        // self.need[tid][rid] += 1;
    }
    /// return the clone of avialiable
    pub fn avialiable(&self) -> Vec<usize> {
        self.avialiable.clone()
    }
}

impl Display for DeadLockDetector {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(f, "avialiable: ")?;
        for avi in self.avialiable.clone() {
            write!(f, "{} ", avi)?;
        }
        writeln!(f)?;
        writeln!(f, "allocation: ")?;
        for allo in self.allocation.clone() {
            for a in allo {
                write!(f, "{} ", a)?;
            }
            writeln!(f)?;
        }
        writeln!(f, "need: ")?;
        for allo in self.need.clone() {
            for a in allo {
                write!(f, "{} ", a)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
