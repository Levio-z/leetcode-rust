/// [1114] Print in Order
///
/// Suppose we have a class:
///
/// public class Foo {
///   public void first() { print("first"); }
///   public void second() { print("second"); }
///   public void third() { print("third"); }
/// }
///
/// The same instance of Foo will be passed to three different threads. Thread A
/// will call first(), thread B will call second(), and thread C will call
/// third(). Design a mechanism and modify the program to ensure that second()
/// is executed after first(), and third() is executed after second(). Note:
/// We do not know how the threads will be scheduled in the operating system,
/// even though the numbers in the input seem to imply the ordering. The input
/// format you see is mainly to ensure our tests' comprehensiveness.  
/// <strong class="example">Example 1:
///
/// Input: nums = [1,2,3]
/// Output: "firstsecondthird"
/// Explanation: There are three threads being fired asynchronously. The input
/// [1,2,3] means thread A calls first(), thread B calls second(), and thread C
/// calls third(). "firstsecondthird" is the correct output.
///
/// <strong class="example">Example 2:
///
/// Input: nums = [1,3,2]
/// Output: "firstsecondthird"
/// Explanation: The input [1,3,2] means thread A calls first(), thread B calls
/// third(), and thread C calls second(). "firstsecondthird" is the correct
/// output.
///
///  
/// Constraints:
///
///     nums is a permutation of [1, 2, 3].
pub struct Solution {}

// problem: https://leetcode.com/problems/print-in-order/
// discuss: https://leetcode.com/problems/print-in-order/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

/// 顺序打印 Trait，定义统一的接口
pub trait PrintInOrderTrait {
    /// 创建新的顺序打印实例
    fn new() -> Self
    where
        Self: Sized;

    /// 执行第一个打印操作
    fn first<F>(&self, print_first: F)
    where
        F: FnOnce();

    /// 执行第二个打印操作（必须在 first 之后）
    fn second<F>(&self, print_second: F)
    where
        F: FnOnce();

    /// 执行第三个打印操作（必须在 second 之后）
    fn third<F>(&self, print_third: F)
    where
        F: FnOnce();
}

use std::sync::{Arc, Condvar, Mutex};

/// 条件变量实现 - 使用互斥锁和条件变量进行线程同步
pub struct CondVarFoo {
    state: Mutex<i32>,
    cv: Condvar,
    cv2: Condvar,
    cv3: Condvar,
}

impl PrintInOrderTrait for CondVarFoo {
    fn new() -> Self {
        CondVarFoo {
            state: Mutex::new(0),
            cv: Condvar::new(),
            cv2: Condvar::new(),
            cv3: Condvar::new(),
        }
    }

    fn first<F>(&self, print_first: F)
    where
        F: FnOnce(),
    {
        // Do not change this line
        let mut state = self.state.lock().unwrap();
        while *state != 0 {
            state = self.cv3.wait(state).unwrap();
        }
        print_first();
        *state += 1;
        self.cv.notify_one();
    }

    fn second<F>(&self, print_second: F)
    where
        F: FnOnce(),
    {
        // Do not change this line
        let mut state = self.state.lock().unwrap();
        while *state != 1 {
            state = self.cv.wait(state).unwrap();
        }

        print_second();

        *state = 2;
        self.cv2.notify_one();
    }

    fn third<F>(&self, print_third: F)
    where
        F: FnOnce(),
    {
        // Do not change this line
        let mut state = self.state.lock().unwrap();
        while *state != 2 {
            state = self.cv2.wait(state).unwrap();
        }

        print_third();
        *state = 0;
        self.cv3.notify_one();
    }
}

use std::sync::mpsc::{Receiver, Sender, channel};

/// 通道实现 - 使用消息传递进行线程同步
pub struct ChannelFoo {
    tx1: Sender<()>,
    rx1: Mutex<Receiver<()>>,
    tx2: Sender<()>,
    rx2: Mutex<Receiver<()>>,
}

impl PrintInOrderTrait for ChannelFoo {
    fn new() -> Self {
        let (tx1, rx1) = channel();
        let (tx2, rx2) = channel();
        ChannelFoo {
            tx1,
            rx1: Mutex::new(rx1),
            tx2,
            rx2: Mutex::new(rx2),
        }
    }

    fn first<F>(&self, print_first: F)
    where
        F: FnOnce(),
    {
        print_first();
        // 通知 second 可以执行
        let _ = self.tx1.send(());
    }

    fn second<F>(&self, print_second: F)
    where
        F: FnOnce(),
    {
        // 等待 first 完成
        let _ = self.rx1.lock().unwrap().recv();
        print_second();
        // 通知 third 可以执行
        let _ = self.tx2.send(());
    }

    fn third<F>(&self, print_third: F)
    where
        F: FnOnce(),
    {
        // 等待 second 完成
        let _ = self.rx2.lock().unwrap().recv();
        print_third();
    }
}

/// 原子变量实现 - 使用原子操作进行无锁同步
use std::sync::atomic::{AtomicI32, Ordering};

pub struct AtomicFoo {
    state: AtomicI32,
}

impl PrintInOrderTrait for AtomicFoo {
    fn new() -> Self {
        AtomicFoo {
            state: AtomicI32::new(0),
        }
    }

    fn first<F>(&self, print_first: F)
    where
        F: FnOnce(),
    {
        // 等待状态为0
        while self.state.load(Ordering::Acquire) != 0 {
            std::thread::yield_now();
        }
        print_first();
        self.state.store(1, Ordering::Release);
    }

    fn second<F>(&self, print_second: F)
    where
        F: FnOnce(),
    {
        // 等待状态为1
        while self.state.load(Ordering::Acquire) != 1 {
            std::thread::yield_now();
        }
        print_second();
        self.state.store(2, Ordering::Release);
    }

    fn third<F>(&self, print_third: F)
    where
        F: FnOnce(),
    {
        // 等待状态为2
        while self.state.load(Ordering::Acquire) != 2 {
            std::thread::yield_now();
        }
        print_third();
        self.state.store(0, Ordering::Release);
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use std::{
        sync::{Arc, Mutex},
        thread,
        time::Duration,
    };

    use rand::Rng;

    use super::*;

    /// 基础功能测试 - 顺序执行
    fn test_basic_operations<T: PrintInOrderTrait>() {
        let foo = T::new();
        let mut output = String::new();
        foo.first(|| output.push_str("first"));
        foo.second(|| output.push_str("second"));
        foo.third(|| output.push_str("third"));
        assert_eq!(output, "firstsecondthird");
    }

    /// 并发功能测试 - 乱序启动线程
    fn test_concurrent_operations<T: PrintInOrderTrait + Sync + Send + 'static>() {
        let foo = Arc::new(T::new());
        let output = Arc::new(Mutex::new(String::new()));

        let f1 = {
            let foo = Arc::clone(&foo);
            let out = Arc::clone(&output);
            thread::spawn(move || {
                foo.first(|| {
                    out.lock().unwrap().push_str("first");
                });
            })
        };

        let f2 = {
            let foo = Arc::clone(&foo);
            let out = Arc::clone(&output);
            thread::spawn(move || {
                foo.second(|| {
                    out.lock().unwrap().push_str("second");
                });
            })
        };

        let f3 = {
            let foo = Arc::clone(&foo);
            let out = Arc::clone(&output);
            thread::spawn(move || {
                foo.third(|| {
                    out.lock().unwrap().push_str("third");
                });
            })
        };

        // 等待所有线程结束
        f1.join().unwrap();
        f2.join().unwrap();
        f3.join().unwrap();

        assert_eq!(&*output.lock().unwrap(), "firstsecondthird");
    }

    /// 性能测试函数
    fn performance_test<T: PrintInOrderTrait + Send + Sync + 'static>(impl_name: &str) {
        println!("\n=== {} 性能测试 ===", impl_name);

        let start = std::time::Instant::now();

        // 测试多次并发执行
        for group in 0..100 {
            let foo = Arc::new(T::new());
            let output = Arc::new(Mutex::new(String::new()));

            let handles: Vec<_> = (0..3)
                .map(|i| {
                    let foo = Arc::clone(&foo);
                    let out = Arc::clone(&output);
                    thread::spawn(move || match i {
                        0 => foo.first(|| {
                            out.lock().unwrap().push_str("first");
                            // println!("first{}", group);
                        }),
                        1 => foo.second(|| {
                            out.lock().unwrap().push_str("second");
                            // println!("second{}", group);
                        }),
                        2 => foo.third(|| {
                            out.lock().unwrap().push_str("third");
                            // println!("third{}", group);
                        }),
                        _ => unreachable!(),
                    })
                })
                .collect();

            for handle in handles {
                handle.join().unwrap();
            }
        }

        let duration = start.elapsed();
        println!("100次并发执行耗时: {:?}", duration);
    }

    /// 压力测试 - 大量线程并发
    fn stress_test<T: PrintInOrderTrait + Send + Sync + 'static>(impl_name: &str) {
        println!("\n=== {} 压力测试 ===", impl_name);

        let start = std::time::Instant::now();
        let foo = Arc::new(T::new());
        let output = Arc::new(Mutex::new(String::new()));

        let mut handles = vec![];

        // 创建多个线程组
        for group in 0..10 {
            let foo = Arc::clone(&foo);
            let out = Arc::clone(&output);
            let foo_clone = Arc::clone(&foo);
            let h1 = thread::spawn(move || {
                foo_clone.first(|| {
                    thread::sleep(Duration::from_millis(rand::thread_rng().gen_range(0..100)));
                    let mut lock = out.lock().unwrap();
                    lock.push_str(&format!("first{}", group));
                });
            });

            let foo = Arc::clone(&foo);
            let out = Arc::clone(&output);

            let foo_clone = Arc::clone(&foo);
            let h2 = thread::spawn(move || {
                foo_clone.second(|| {
                    thread::sleep(Duration::from_millis(rand::thread_rng().gen_range(0..100)));
                    let mut lock = out.lock().unwrap();
                    lock.push_str(&format!("second{}", group));
                });
            });

            let foo = Arc::clone(&foo);
            let out = Arc::clone(&output);

            let h3 = thread::spawn(move || {
                foo.third(|| {
                    thread::sleep(Duration::from_millis(rand::thread_rng().gen_range(0..100)));
                    let mut lock = out.lock().unwrap();
                    lock.push_str(&format!("third{}", group));
                });
            });

            handles.push(h1);
            handles.push(h2);
            handles.push(h3);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let duration = start.elapsed();
        println!("30个线程并发执行耗时: {:?}", duration);

        // 验证输出顺序,保证不了
        let result = output.lock().unwrap();
        println!("Actual: {}", result);
        // for group in 0..10 {
        //     let expected = format!("first{}second{}third{}", group, group,
        // group);     println!("Expected: {}", expected);
        //     assert!(
        //         result.contains(&expected),
        //         "Group {} output incorrect",
        //         group
        //     );
        // }
    }

    #[test]
    fn test_condvar_implementation() {
        test_basic_operations::<CondVarFoo>();
        test_concurrent_operations::<CondVarFoo>();
        println!("条件变量实现测试通过");
    }

    #[test]
    fn test_channel_implementation() {
        test_basic_operations::<ChannelFoo>();
        test_concurrent_operations::<ChannelFoo>();
        println!("通道实现测试通过");
    }

    #[test]
    fn test_atomic_implementation() {
        test_basic_operations::<AtomicFoo>();
        test_concurrent_operations::<AtomicFoo>();
        println!("原子变量实现测试通过");
    }

    #[test]
    fn compare_all_implementations() {
        // 运行性能测试
        performance_test::<CondVarFoo>("条件变量实现");
        performance_test::<ChannelFoo>("通道实现");
        performance_test::<AtomicFoo>("原子变量实现");

        // 运行压力测试,多线程不行，只能保证当线程的顺序执行，不能保证多个线程并发执行
        // 各个线程的信号会互相干扰
        stress_test::<CondVarFoo>("条件变量实现");
        stress_test::<ChannelFoo>("通道实现");
        stress_test::<AtomicFoo>("原子变量实现");

        println!("\n=== 实现方案对比总结 ===");
        println!("1. 条件变量实现: 传统同步方式，适合复杂同步场景");
        println!("2. 通道实现: 消息传递模式，逻辑清晰，适合简单同步");
        println!("3. 原子变量实现: 无锁同步，性能高，但需要忙等待");
    }

    /// 测试不同执行顺序
    #[test]
    fn test_different_orders() {
        // 测试顺序 1-2-3
        let foo = Arc::new(CondVarFoo::new());
        let mut output = Arc::new(Mutex::new(String::new()));
        let foo1 = Arc::clone(&foo);
        let output1 = Arc::clone(&output);
        let h1 = thread::spawn(move || {
            foo1.first(|| output1.lock().unwrap().push_str("first"));
        });
        let foo2 = Arc::clone(&foo);
        let output2 = Arc::clone(&output);
        let h2 = thread::spawn(move || {
            foo2.second(|| output2.lock().unwrap().push_str("second"));
        });
        let foo3 = Arc::clone(&foo);
        let output3 = Arc::clone(&output);
        let h3 = thread::spawn(move || {
            foo3.third(|| output3.lock().unwrap().push_str("third"));
        });
        h1.join().unwrap();
        h2.join().unwrap();
        h3.join().unwrap();
        assert_eq!(output.lock().unwrap().as_str(), "firstsecondthird");

        // 测试顺序 1-3-2（应该仍然是 first-second-third）
        let foo = Arc::new(CondVarFoo::new());
        let mut output = Arc::new(Mutex::new(String::new()));
        let foo1 = Arc::clone(&foo);
        let output1 = Arc::clone(&output);
        let h1 = thread::spawn(move || {
            foo1.first(|| output1.lock().unwrap().push_str("first"));
        });
        let foo2 = Arc::clone(&foo);
        let output2 = Arc::clone(&output);
        let h2 = thread::spawn(move || {
            foo2.third(|| output2.lock().unwrap().push_str("third"));
        });
        let foo3 = Arc::clone(&foo);
        let output3 = Arc::clone(&output);
        let h3 = thread::spawn(move || {
            foo3.second(|| output3.lock().unwrap().push_str("second"));
        });
        h1.join().unwrap();
        h2.join().unwrap();
        h3.join().unwrap();
        assert_eq!(output.lock().unwrap().as_str(), "firstsecondthird");

        // 测试顺序 2-1-3（应该仍然是 first-second-third）
        let foo = Arc::new(CondVarFoo::new());
        let mut output = Arc::new(Mutex::new(String::new()));
        let foo1 = Arc::clone(&foo);
        let output1 = Arc::clone(&output);
        let h1 = thread::spawn(move || {
            foo1.second(|| output1.lock().unwrap().push_str("second"));
        });
        let foo2 = Arc::clone(&foo);
        let output2 = Arc::clone(&output);
        let h2 = thread::spawn(move || {
            foo2.first(|| output2.lock().unwrap().push_str("first"));
        });
        let foo3 = Arc::clone(&foo);
        let output3 = Arc::clone(&output);
        let h3 = thread::spawn(move || {
            foo3.third(|| output3.lock().unwrap().push_str("third"));
        });
        h1.join().unwrap();
        h2.join().unwrap();
        h3.join().unwrap();

        assert_eq!(output.lock().unwrap().as_str(), "firstsecondthird");
    }
}
