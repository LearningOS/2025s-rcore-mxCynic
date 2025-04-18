# lab1

## 实现新得

### 初步实现
```
// TODO: implement the syscall
pub fn sys_trace(trace_request: usize, id: usize, data: usize) -> isize {
    trace!("kernel: sys_trace");
    match trace_request {
        0 => unsafe { *(id as *const u8) as isize },
        1 => {
            unsafe {
                *(id as *mut u8) = data as u8;
            };
            0
        }
        2 => TODO!(),
        _ => -1,
    }
}
```
初步我通过match不同的trace_request来完成不同的需求。其中trace_request值为2时，即查询操作时，
是最麻烦的。
### 补充细节
在实现这个的过程中，我首先想到的就是维护一个static的初始值全为0的数组，通过syscall id的值来访问这个数组。
这时候就遇到了第一个问题，那就是数组的大小是多少,我发现了os项目下的src/syscall/mod.rs文件。里面写了部分
的syscall id。后面又发现了usr测试项目下的src/syscall.rs文件中写了全部的id。他们中的最大值为473。凑了个整，
我将数组设置为500的大小。
```
/// 系统调用id的最大值
const MAX_SYSCALL_NUM: usize = 500;

```
接下来的问题是这个静态数组应该放在什么位置，我一开始在sys_trace内设置了这个静态
数组,这是明显不对的，只是我的初步尝试。后面看到实现提示中的扩展 TaskManagerInner 中的结构。因此我将TaskManagerInner
类型改造成
```
pub struct TaskManagerInner {
    /// task list
    tasks: [TaskControlBlock; MAX_APP_NUM],
    /// id of current `Running` task
    current_task: usize,
    calltimes: [0: MAX_SYSCALL_NUM],
}
```
我为其设置了一个calltime_add(id: usize)函数来使得syscall对应的id增加。
还有一个calltime(id: usize)函数来查询对应的syscall的值。
其中calltime_add()是放在sys/syscall/mod.rs中的syscall中的
```
/// handle syscall exception with `syscall_id` and other arguments
pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    calltime_add(syscall_id);  // new add
    match syscall_id {
        ...
    }
}
```
这是因为内核态在不论调用何种call都会通过这个函数，因此将增加这个行为放在这里。
而查询则简单很多。
```
pub fn sys_trace(trace_request: usize, id: usize, data: usize) -> isize {
    trace!("kernel: sys_trace");
    match trace_request {
        ...
        2 => calltime(id),
        ...
    }
}
```
此时运行测试，我发现测试代码user/src/bin/ch3_trace.rs中的
```
    assert_eq!(0, count_syscall(SYSCALL_WRITE));
```
这个的实际值为5。在查询gpt后，得到信息是某些print会调用sys_write。但是这个测试用例
并没有有print语句。  
在后续的思考中，我意识到。我的calltimes数组是TASKMANAGER层面的，也就是说所有的app都
会共享到同一个calltimes数组,这就会导致其他的测试用例调用的syscall信息会保存到报错的
ch3_trace.rs中，因此在最终的设计中我将calltimes移到了TaskControBlock中
```
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
    /// syscall times  new add
    pub call_times: [isize; MAX_SYSCALL_NUM],
}
```
其他的地方对应的进行小幅度的修改。最终就通过测试了。




## 简答作业

## 荣誉准则


1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

        《你交流的对象说明》

2. 此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

        《你参考的资料说明》

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。
