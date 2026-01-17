# 交替打印FooBar - 多实现对比

## 1. 问题描述

LeetCode 1115 交替打印FooBar问题要求实现一个类，该类有两个方法`foo()`和`bar()`，分别输出"foo"和"bar"。这两个方法将被两个不同的线程调用，要求它们交替执行，最终输出"foobar"重复n次。

**输入**：n = 1  
**输出**："foobar"  

**输入**：n = 2  
**输出**："foobarfoobar"

**约束条件**：

- 1 <= n <= 1000

## 2. 设计思路

### 2.1 核心问题分析

这个问题的核心是线程同步，需要确保：

1. `foo()`方法必须在`bar()`方法之前执行
2. 每个"foo"之后必须跟着一个"bar"
3. 整个序列需要重复n次

### 2.2 Trait抽象设计

为了支持多种实现方式，我们定义了一个`PrintFooBar` trait，统一接口：

```rust
trait PrintFooBar {
    fn new(n: usize) -> Self;
    
    fn foo<F>(&self, print_foo: F)
    where
        F: Fn();
    
    fn bar<F>(&self, print_bar: F)
    where
        F: Fn();
}
```

## 3. 实现方案

### 3.1 基于Condvar的实现

```rust
struct CondvarFooBar {
    n: usize,
    state: Mutex<i32>,
    cv: Condvar,
}

impl PrintFooBar for CondvarFooBar {
    fn new(n: usize) -> Self {
        CondvarFooBar {
            n,
            state: Mutex::new(0),
            cv: Condvar::new(),
        }
    }

    fn foo<F>(&self, print_foo: F)
    where
        F: Fn(),
    {
        for _ in 0..self.n {
            let mut state = self.state.lock().unwrap();
            while *state != 0 {
                state = self.cv.wait(state).unwrap();
            }
            print_foo();
            *state += 1;
            self.cv.notify_one();
        }
    }

    fn bar<F>(&self, print_bar: F)
    where
        F: Fn(),
    {
        for _ in 0..self.n {
            let mut state = self.state.lock().unwrap();
            while *state != 1 {
                state = self.cv.wait(state).unwrap();
            }
            print_bar();
            *state -= 1;
            self.cv.notify_one();
        }
    }
}
```

**工作原理**：

- 使用`Mutex`保护共享状态`state`（0表示可以打印"foo"，1表示可以打印"bar"）
- 使用`Condvar`实现线程间的等待和通知机制
- `foo()`方法等待`state`为0，打印后将`state`设置为1并通知等待的线程
- `bar()`方法等待`state`为1，打印后将`state`重置为0并通知等待的线程

### 3.2 基于Barrier的实现

```rust
struct BarrierFooBar {
    n: usize,
    barrier: Barrier,
}

impl PrintFooBar for BarrierFooBar {
    fn new(n: usize) -> Self {
        BarrierFooBar {
            n,
            barrier: Barrier::new(2),
        }
    }

    fn foo<F>(&self, print_foo: F)
    where
        F: Fn(),
    {
        for _ in 0..self.n {
            print_foo();
            self.barrier.wait();
            self.barrier.wait();
        }
    }

    fn bar<F>(&self, print_bar: F)
    where
        F: Fn(),
    {
        for _ in 0..self.n {
            self.barrier.wait();
            print_bar();
            self.barrier.wait();
        }
    }
}
```

**工作原理**：

- 使用`Barrier`创建一个线程同步点，要求2个线程都到达后才能继续
- 每次迭代中，两个线程都等待第一个屏障，确保它们都准备好
- 先到达的线程会阻塞，直到另一个线程也到达
- 打印完成后，再次等待屏障，确保两个线程都完成当前步骤

## 4. 性能分析

### 4.1 时间复杂度

两种实现的时间复杂度都是O(n)，因为它们都需要执行n次打印操作。

### 4.2 空间复杂度

- `CondvarFooBar`：O(1)，只需要固定大小的锁和条件变量
- `BarrierFooBar`：O(1)，只需要一个固定大小的屏障

### 4.3 性能比较

| 实现方式 | 优点 | 缺点 | 适用场景 |
| --------- | ------ | ------ | --------- |
| Condvar | 精确控制执行顺序，资源消耗低 | 实现相对复杂，需要手动管理状态 | 对执行顺序有严格要求的场景 |
| Barrier | 实现简洁，代码可读性高 | 无法控制哪个线程先执行，只能确保同步 | 只需要同步执行点，不关心执行顺序的场景 |

## 5. Rust实现特色

### 5.1 泛型和闭包

使用泛型和闭包允许灵活地传递打印函数，提高代码的复用性和可测试性。

### 5.2 所有权管理

利用Rust的所有权系统确保线程安全，避免数据竞争。

### 5.3 错误处理

使用`unwrap()`简化错误处理，在测试环境中是可接受的。在生产环境中，可以考虑更优雅的错误处理方式。

## 6. 测试设计

### 6.1 基本功能测试

测试不同n值下的基本交替打印功能：

- n=1时输出"foobar"
- n=2时输出"foobarfoobar"
- n=10时输出10次"foobar"

### 6.2 并发执行测试

模拟实际场景，测试多个并发执行：

```rust
for _ in 0..100 {
    test_implementation::<CondvarFooBar>(5);
    test_implementation::<BarrierFooBar>(5);
}
```

### 6.3 性能测试

测试在约束条件上限（n=1000）下的性能表现。

## 7. 扩展思考

### 7.1 其他实现方式

除了已实现的两种方式，还可以考虑：

- 使用原子变量（AtomicBool）
- 使用通道（mpsc）
- 使用信号量（Semaphore）

### 7.2 相关LeetCode问题

- [1114. 按序打印](https://leetcode.com/problems/print-in-order/)：类似的线程同步问题
- [1116. 打印零与奇偶数](https://leetcode.com/problems/print-zero-even-odd/)：更复杂的多线程交替打印问题

### 7.3 实际应用场景

- 多线程任务调度
- 生产者-消费者模型
- GUI界面更新与后台数据处理的同步
- 分布式系统中的协调机制

## 8. 总结

LeetCode 1115问题展示了多线程同步的经典场景。通过trait抽象，我们可以轻松比较不同的实现方式：

1. **Condvar实现**：提供精确的执行顺序控制，适合对顺序有严格要求的场景
2. **Barrier实现**：代码简洁，适合只需要同步执行点的场景

两种实现都具有O(n)的时间复杂度和O(1)的空间复杂度，但在实际应用中需要根据具体需求选择合适的方案。Rust的所有权系统和并发原语为实现高效、安全的多线程程序提供了强大支持。
