/// [1472] Design Browser History
///
/// You have a browser of one tab where you start on the homepage and you can
/// visit another url, get back in the history number of steps or move forward
/// in the history number of steps. Implement the BrowserHistory class:
///
/// BrowserHistory(string homepage) Initializes the object with the homepage of
/// the browser. void visit(string url) Visits url from the current page. It
/// clears up all the forward history. string back(int steps) Move steps back in
/// history. If you can only return x steps in the history and steps > x, you
/// will return only x steps. Return the current url after moving back in
/// history at most steps. string forward(int steps) Move steps forward in
/// history. If you can only forward x steps in the history and steps > x, you
/// will forward only x steps. Return the current url after forwarding in
/// history at most steps.
///
///  
/// <strong class="example">Example:
///
/// Input:
/// ["BrowserHistory","visit","visit","visit","back","back","forward","visit","
/// forward","back","back"] [["leetcode.com"],["google.com"],["facebook.com"],["
/// youtube.com"],[1],[1],[1],["linkedin.com"],[2],[2],[7]] Output:
/// [null,null,null,null,"facebook.com","google.com","facebook.com",null,"
/// linkedin.com","google.com","leetcode.com"] Explanation:
/// BrowserHistory browserHistory = new BrowserHistory("leetcode.com");
/// browserHistory.visit("google.com");       // You are in "leetcode.com".
/// Visit "google.com" browserHistory.visit("facebook.com");     // You are in
/// "google.com". Visit "facebook.com" browserHistory.visit("youtube.com");
/// // You are in "facebook.com". Visit "youtube.com" browserHistory.back(1);
/// // You are in "youtube.com", move back to "facebook.com" return
/// "facebook.com" browserHistory.back(1);                   // You are in
/// "facebook.com", move back to "google.com" return "google.com"
/// browserHistory.forward(1);                // You are in "google.com", move
/// forward to "facebook.com" return "facebook.com" browserHistory.visit("
/// linkedin.com");     // You are in "facebook.com". Visit "linkedin.com"
/// browserHistory.forward(2);                // You are in "linkedin.com", you
/// cannot move forward any steps. browserHistory.back(2);                   //
/// You are in "linkedin.com", move back two steps to "facebook.com" then to
/// "google.com". return "google.com" browserHistory.back(7);
/// // You are in "google.com", you can move back only one step to
/// "leetcode.com". return "leetcode.com"
///
///  
/// Constraints:
///
/// 1 <= homepage.length <= 20
/// 1 <= url.length <= 20
/// 1 <= steps <= 100
/// homepage and url consist of  '.' or lower case English letters.
/// At most 5000 calls will be made to visit, back, and forward.
pub struct Solution {}

// problem: https://leetcode.com/problems/design-browser-history/
// discuss: https://leetcode.com/problems/design-browser-history/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

/// 浏览器历史记录trait，定义统一的接口
pub trait BrowserHistoryTrait {
    /// 创建新的浏览器历史记录
    fn new(homepage: String) -> Self
    where
        Self: Sized;

    /// 访问新的URL，清空前向历史
    fn visit(&mut self, url: String);

    /// 向后移动steps步，返回当前URL
    fn back(&mut self, steps: i32) -> String;

    /// 向前移动steps步，返回当前URL
    fn forward(&mut self, steps: i32) -> String;
}

/// 数组实现 - 使用向量和双指针
#[derive(Debug)]
pub struct ArrayBrowserHistory {
    history: Vec<String>,
    cur: i32,
    end: i32,
}

impl BrowserHistoryTrait for ArrayBrowserHistory {
    fn new(homepage: String) -> Self {
        Self {
            history: vec![homepage],
            cur: 0,
            end: 0,
        }
    }

    fn visit(&mut self, url: String) {
        self.cur += 1;
        self.end = self.cur;
        let cur = self.cur as usize;
        if cur >= self.history.len() {
            self.history.push(url);
        } else {
            self.history[cur] = url;
        }
    }

    fn back(&mut self, steps: i32) -> String {
        self.cur = (self.cur - steps).max(0);
        self.history[self.cur as usize].clone()
    }

    fn forward(&mut self, steps: i32) -> String {
        self.cur = (self.cur + steps).min(self.end);
        self.history[self.cur as usize].clone()
    }
}

/// 双栈实现 - 使用两个栈分别存储后退和前向历史
#[derive(Debug)]
pub struct StackBrowserHistory {
    back_stack: Vec<String>,
    forward_stack: Vec<String>,
}

impl BrowserHistoryTrait for StackBrowserHistory {
    fn new(homepage: String) -> Self {
        let mut bh = StackBrowserHistory {
            back_stack: Vec::new(),
            forward_stack: Vec::new(),
        };
        bh.back_stack.push(homepage);
        bh
    }

    fn visit(&mut self, url: String) {
        self.forward_stack.clear();
        self.back_stack.push(url);
    }

    fn back(&mut self, steps: i32) -> String {
        let times = if steps >= self.back_stack.len() as i32 {
            self.back_stack.len() - 1
        } else {
            steps as usize
        };

        for _ in 0..times {
            self.forward_stack.push(self.back_stack.pop().unwrap());
        }

        self.back_stack.last().unwrap().to_string()
    }

    fn forward(&mut self, steps: i32) -> String {
        let times = if steps >= self.forward_stack.len() as i32 {
            self.forward_stack.len()
        } else {
            steps as usize
        };

        for _ in 0..times {
            self.back_stack.push(self.forward_stack.pop().unwrap());
        }

        self.back_stack.last().unwrap().to_string()
    }
}

/// 截断数组实现 - 使用向量和当前索引，访问时截断历史
#[derive(Debug)]
pub struct TruncateArrayBrowserHistory {
    urls: Vec<String>,
    curr_index: usize,
}

impl BrowserHistoryTrait for TruncateArrayBrowserHistory {
    fn new(homepage: String) -> Self {
        TruncateArrayBrowserHistory {
            urls: vec![homepage],
            curr_index: 0,
        }
    }

    fn visit(&mut self, url: String) {
        self.urls.truncate(self.curr_index + 1);
        self.urls.push(url);
        self.curr_index += 1;
    }

    fn back(&mut self, steps: i32) -> String {
        self.curr_index = std::cmp::max(self.curr_index as i32 - steps, 0) as usize;
        self.urls[self.curr_index].clone()
    }

    fn forward(&mut self, steps: i32) -> String {
        self.curr_index = std::cmp::min(self.curr_index + steps as usize, self.urls.len() - 1);
        self.urls[self.curr_index].clone()
    }
}

// submission codes end
#[cfg(test)]
mod tests {
    use std::time::Instant;

    use super::*;

    /// 基础功能测试
    fn test_basic_operations<T: BrowserHistoryTrait>() {
        let mut browser_history = T::new("leetcode.com".to_string());
        browser_history.visit("google.com".to_string());
        browser_history.visit("facebook.com".to_string());
        browser_history.visit("youtube.com".to_string());

        assert_eq!(browser_history.back(1), "facebook.com".to_string());
        assert_eq!(browser_history.back(1), "google.com".to_string());
        assert_eq!(browser_history.forward(1), "facebook.com".to_string());

        browser_history.visit("linkedin.com".to_string());
        assert_eq!(browser_history.forward(2), "linkedin.com".to_string());
        assert_eq!(browser_history.back(2), "google.com".to_string());
        assert_eq!(browser_history.back(7), "leetcode.com".to_string());
    }

    /// 边界条件测试
    fn test_edge_cases<T: BrowserHistoryTrait>() {
        let mut browser_history = T::new("homepage.com".to_string());

        // 测试边界后退
        assert_eq!(browser_history.back(100), "homepage.com".to_string());

        // 测试边界前进
        assert_eq!(browser_history.forward(100), "homepage.com".to_string());

        // 测试连续访问
        for i in 0..10 {
            browser_history.visit(format!("site{}.com", i));
        }

        // 测试部分后退和前进
        assert_eq!(browser_history.back(5), "site4.com".to_string());
        assert_eq!(browser_history.forward(2), "site6.com".to_string());
    }

    /// 性能测试函数
    fn performance_test<T: BrowserHistoryTrait + 'static>(impl_name: &str) {
        println!("\n=== {} 性能测试 ===", impl_name);

        // 测试大量访问操作
        let start = Instant::now();
        let mut browser_history = T::new("start.com".to_string());
        for i in 0..2000 {
            browser_history.visit(format!("site{}.com", i));
        }
        let visit_time = start.elapsed();
        println!("1000次访问操作耗时: {:?}", visit_time);

        // 测试大量后退操作
        let start = Instant::now();
        for _ in 0..2000 {
            browser_history.back(2);
            browser_history.forward(1);
        }
        let navigation_time = start.elapsed();
        println!("500次导航操作耗时: {:?}", navigation_time);

        // 测试混合操作
        let start = Instant::now();
        for i in 0..1000 {
            browser_history.visit(format!("newsite{}.com", i));
            browser_history.back(1);
            browser_history.forward(1);
        }
        let mixed_time = start.elapsed();
        println!("200次混合操作耗时: {:?}", mixed_time);

        println!("总内存使用情况: 需要手动检查");
    }

    #[test]
    fn test_array_implementation() {
        test_basic_operations::<ArrayBrowserHistory>();
        test_edge_cases::<ArrayBrowserHistory>();
        println!("数组实现测试通过");
    }

    #[test]
    fn test_stack_implementation() {
        test_basic_operations::<StackBrowserHistory>();
        test_edge_cases::<StackBrowserHistory>();
        println!("双栈实现测试通过");
    }

    #[test]
    fn test_truncate_array_implementation() {
        test_basic_operations::<TruncateArrayBrowserHistory>();
        test_edge_cases::<TruncateArrayBrowserHistory>();
        println!("截断数组实现测试通过");
    }

    #[test]
    fn compare_performance() {
        // 运行性能测试
        performance_test::<ArrayBrowserHistory>("数组实现");
        performance_test::<StackBrowserHistory>("双栈实现");
        performance_test::<TruncateArrayBrowserHistory>("截断数组实现");
        performance_test::<StackBrowserHistory>("双栈实现");
        performance_test::<ArrayBrowserHistory>("数组实现");
        performance_test::<TruncateArrayBrowserHistory>("截断数组实现");
        performance_test::<ArrayBrowserHistory>("数组实现");
        performance_test::<StackBrowserHistory>("双栈实现");
        performance_test::<TruncateArrayBrowserHistory>("截断数组实现");

        println!("\n=== 性能分析总结 ===");
        println!("1. 数组实现: 内存连续访问，适合频繁导航操作");
        println!("2. 双栈实现: 后退前进操作高效，但内存使用可能较高");
        println!("3. 截断数组实现: 内存使用优化，但截断操作有开销");
    }

    /// 示例用法测试
    #[test]
    fn test_example_usage() {
        // 使用数组实现
        let mut array_impl = ArrayBrowserHistory::new("leetcode.com".to_string());
        array_impl.visit("google.com".to_string());
        assert_eq!(array_impl.back(1), "leetcode.com".to_string());

        // 使用双栈实现
        let mut stack_impl = StackBrowserHistory::new("leetcode.com".to_string());
        stack_impl.visit("google.com".to_string());
        assert_eq!(stack_impl.back(1), "leetcode.com".to_string());

        // 使用截断数组实现
        let mut truncate_impl = TruncateArrayBrowserHistory::new("leetcode.com".to_string());
        truncate_impl.visit("google.com".to_string());
        assert_eq!(truncate_impl.back(1), "leetcode.com".to_string());
    }
}
