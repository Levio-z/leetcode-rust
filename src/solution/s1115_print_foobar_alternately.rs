/// [1115] Print FooBar Alternately
///
/// Suppose you are given the following code:
///
/// class FooBar {
///   public void foo() {
///     for (int i = 0; i < n; i++) {
///       print("foo");
///     }
///   }
///   public void bar() {
///     for (int i = 0; i < n; i++) {
///       print("bar");
///     }
///   }
/// }
///
/// The same instance of FooBar will be passed to two different threads:
///
///     thread A will call foo(), while
///     thread B will call bar().
///
/// Modify the given program to output "foobar" n times.
///  
/// <strong class="example">Example 1:
///
/// Input: n = 1
/// Output: "foobar"
/// Explanation: There are two threads being fired asynchronously. One of them
/// calls foo(), while the other calls bar(). "foobar" is being output 1 time.
///
/// <strong class="example">Example 2:
///
/// Input: n = 2
/// Output: "foobarfoobar"
/// Explanation: "foobar" is being output 2 times.
///
///  
/// Constraints:
///
///     1 <= n <= 1000
pub struct Solution {}

// problem: https://leetcode.com/problems/print-foobar-alternately/
// discuss: https://leetcode.com/problems/print-foobar-alternately/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::sync::{Barrier, Condvar, Mutex};

/// Trait defining the interface for alternating FooBar printing
trait PrintFooBar {
    /// Creates a new instance with n iterations
    fn new(n: usize) -> Self;

    /// Prints "foo" n times
    fn foo<F>(&self, print_foo: F)
    where
        F: Fn();

    /// Prints "bar" n times, alternating with foo
    fn bar<F>(&self, print_bar: F)
    where
        F: Fn();
}

/// Implementation using Mutex and Condvar
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
            // printFoo() outputs "foo". Do not change or remove this line.
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
            // printBar() outputs "bar". Do not change or remove this line.
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

/// Implementation using Barrier
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

// submission codes end

#[cfg(test)]
mod tests {
    use std::{
        sync::{Arc, Mutex},
        thread,
    };

    use super::*;

    /// Test helper function to test a specific PrintFooBar implementation
    fn test_implementation<T: PrintFooBar + Send + Sync + 'static>(n: usize) {
        let foo_bar = Arc::new(T::new(n));
        let output = Arc::new(Mutex::new(String::new()));

        let foo_output = Arc::clone(&output);
        let foo_clone = Arc::clone(&foo_bar);
        let handle_foo = thread::spawn(move || {
            foo_clone.foo(|| {
                foo_output.lock().unwrap().push_str("foo");
            });
        });

        let bar_output = Arc::clone(&output);
        let bar_clone = Arc::clone(&foo_bar);
        let handle_bar = thread::spawn(move || {
            bar_clone.bar(|| {
                bar_output.lock().unwrap().push_str("bar");
            });
        });

        handle_foo.join().unwrap();
        handle_bar.join().unwrap();

        let expected = "foobar".repeat(n);
        assert_eq!(*output.lock().unwrap(), expected);
    }

    #[test]
    fn test_condvar_implementation() {
        test_implementation::<CondvarFooBar>(1);
        test_implementation::<CondvarFooBar>(2);
        test_implementation::<CondvarFooBar>(3);
        test_implementation::<CondvarFooBar>(10);
    }

    #[test]
    fn test_barrier_implementation() {
        test_implementation::<BarrierFooBar>(1);
        test_implementation::<BarrierFooBar>(2);
        test_implementation::<BarrierFooBar>(3);
        test_implementation::<BarrierFooBar>(10);
    }

    #[test]
    fn test_concurrent_execution() {
        // Test both implementations with 100 concurrent executions
        for _ in 0..100 {
            test_implementation::<CondvarFooBar>(5);
            test_implementation::<BarrierFooBar>(5);
        }
    }

    #[test]
    fn test_large_input() {
        // Test with the maximum constraint
        test_implementation::<CondvarFooBar>(1000);
        test_implementation::<BarrierFooBar>(1000);
    }
}
