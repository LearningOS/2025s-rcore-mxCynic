//! Types related to task management

use super::TaskContext;

/// 系统调用id的最大值
const MAX_SYSCALL_NUM: usize = 500;

/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
    /// syscall times
    pub call_times: [isize; MAX_SYSCALL_NUM],
}

/// The status of a task
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}

impl TaskControlBlock {
    /// syscall time 次数加一
    pub fn calltime_add(&mut self, syscall_id: usize) {
        self.call_times[syscall_id] += 1;
    }

    /// 查询 syscalltime
    pub fn calltime(&self, syscall_id: usize) -> isize {
        self.call_times[syscall_id]
    }
}
