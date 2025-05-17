# lab1

## 实现新得


我了解到了用户态和内核态的通信方式.大致理解如下：  
用户态一个Call（如Write），他会调用用户态的syscall，这个用户态的syscall通过ecall
负责和内核态通信，内核太通过trap_handler函数来捕获用户态的信息。通过match来匹配接
受的信息。如果是一个syscall，那么就会调用内核态的syscall。然后在这个syscall中再去
匹配对应的call的具体方式。  
也就是说，要记录题目的要求的调用次数，在内核态的syscall这个位置是最合理的，因为每
一个call都会通过这里，再到具体的callfunc。




## 简答作业

### 1.

我的rustsbi版本是[RustSBI-QEMU 0.1.1](https://github.com/rustsbi/rustsbi-qemu/releases/tag/v0.1.1)
报错显示，
```
[kernel] Panicked at src/syscall/fs.rs:11 called `Result::unwrap()` on an `Err` value: Utf8Error { valid_up_to: 3, error_len: Some(1) }
```
通过查看源代码，我发现这是在sys_write函数内的报错，具体来说是从utf-8读取str值，但是
输入并不是期望的utf-8。查看测试代码发现确实不是合法的utf-8字符
### 2.

#### 1 
刚进入 __restore 时，sp的值是_alltraps中的addi sp, sp -34*8的值，即sp = sp + (-34 * 8),
_alltraps中只在前两行改变了sp的值，后续的操作都只用过sp的值，而没有改变他。  
__restore的两种情况包括  
1. 恢复当前程序，比如用户态运行了一个write指令后，在内核态运行完成后，会通过_restore来
恢复用户态的状态，再去执行用户态的下一个指令。
2. 运行下一个程序,再用户态运行结束，中断等指令时，在内核态接受到这些信息后，就可以通过
__restore来运行下一个程序。

#### 2 

这几行代码用了一下寄存器: t0, t1, t2, sstatus, sepc, sscratch。通过查询riscv手册，其中  
* t0-t2都是temporary寄存器，即是临时变量，可以自由使用。
* sstatus，负责记录cpu在S模式下的状态
* sepc记录发生异常或中断时用户程序要返回的地址
* sscratc记录陷入内核前保存用户态的 sp（栈指针）  

后面三个都是控制状态寄存器
也就是说在_alltraps中通过ld指令(将三个状态控制寄存器的值读取到temporary寄存器)然后在
__restore中通过csrw指令(写控制器指令)将三个temporary寄存器的值写如控制器寄存器。

#### 3 
跳过x2是因为x2就是sp寄存器，sp在trap.S中其他的地方用了。  
跳过x4是因为x4就是tp寄存器，我在trap.S中发现了这个注释
    # skip tp(x4), application does not use it
这就是跳过x4的原因

#### 4 

csrrw指令是csr{r,w}即scrr和scrw两个指令的合并。
```
csrrw rd, csr, rs1
```
他的作用是将rs1的值写如csr，将csr的原值写如rd。
那么对于题目中的
```
csrrw sp, sscratch, sp
```
他的作用就是交换sp和sscratch的值，而这两个寄存器，sp一开始记录的是用户态信息
sscratch负责记录内核态的信息
对于第60行的这个指令，即_restore中该指令
完成了如下转化

```
sp      记录用户态 -> 记录内核态
sscratch记录内核态 -> 记录用户态

```

#### 5

是最后一条指令:
```
sret
```
可以在riscv手册看到这个指令的介绍是:  
sret ExceptionReturn(Supervisor)  
管理员模式例外返回(Supervisor-mode Exception Return). R-type, RV32I and RV64I 特权指令。
从管理员模式的例外处理程序中返回，设置 pc 为 CSRs[spec]，权限模式为 CSRs[sstatus].SPP，
CSRs[sstatus].SIE 为 CSRs[sstatus].SPIE，CSRs[sstatus].SPIE 为 1，CSRs[sstatus].spp 为 0。

#### 6 
和第四题类似，_alltraps中是
```
sp      记录内核态 -> 记录用户态
sscratch记录用户态 -> 记录内核态
```

总的来说，sp 在 __alltraps 中从 sscratch 得到内核态栈地址来保存上下文，sscratch 暂时
存下用户栈指针；在 __restore 的最后再交换回来，把用户栈恢复到 sp，并把 sscratch 更新为新的内核态栈指针，以备下次使用。

#### 7

是用户的ecall完成的





## 荣誉准则


1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

        《你交流的对象说明》
        我在实验中多次与gpt等人工智能交流

2. 此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

        《你参考的资料说明》
        《2018 RISC-V 手册》

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。
