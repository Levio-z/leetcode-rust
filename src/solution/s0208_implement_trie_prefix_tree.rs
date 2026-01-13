/// [208] Implement Trie (Prefix Tree)
///
/// A <a href="https://en.wikipedia.org/wiki/Trie" target="_blank">trie</a> (pronounced as "try") or prefix tree is a tree data structure used to efficiently store and retrieve keys in a dataset of strings. There are various applications of this data structure, such as autocomplete and spellchecker.
/// Implement the Trie class:
///
/// Trie() Initializes the trie object.
/// void insert(String word) Inserts the string word into the trie.
/// boolean search(String word) Returns true if the string word is in the trie
/// (i.e., was inserted before), and false otherwise. boolean startsWith(String
/// prefix) Returns true if there is a previously inserted string word that has
/// the prefix prefix, and false otherwise.
///
///  
/// <strong class="example">Example 1:
///
/// Input
/// ["Trie", "insert", "search", "search", "startsWith", "insert", "search"]
/// [[], ["apple"], ["apple"], ["app"], ["app"], ["app"], ["app"]]
/// Output
/// [null, null, true, false, true, null, true]
/// Explanation
/// Trie trie = new Trie();
/// trie.insert("apple");
/// trie.search("apple");   // return True
/// trie.search("app");     // return False
/// trie.startsWith("app"); // return True
/// trie.insert("app");
/// trie.search("app");     // return True
///
///  
/// Constraints:
///
/// 1 <= word.length, prefix.length <= 2000
/// word and prefix consist only of lowercase English letters.
/// At most 3 * 10^4 calls in total will be made to insert, search, and
/// startsWith.
pub struct Solution {}

// problem: https://leetcode.com/problems/implement-trie-prefix-tree/
// discuss: https://leetcode.com/problems/implement-trie-prefix-tree/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
#[derive(Debug)]
struct Trie {
    root: TrieNode,
}

/// `&self` means the method takes an immutable reference.
/// If you need a mutable reference, change it to `&mut self` instead.
#[derive(Debug)]
struct TrieNode {
    children: [Option<Box<TrieNode>>; 26],
    is_end: bool,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: Default::default(),
            is_end: false,
        }
    }
}

impl Trie {
    fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }

    fn insert(&mut self, word: String) {
        // 获取可变引用
        let mut node = &mut self.root;
        for c in word.bytes() {
            let index = (c - b'a') as usize;
            node = node.children[index].get_or_insert(Box::new(TrieNode::new()));
        }
        node.is_end = true;
    }

    fn search(&self, word: String) -> bool {
        self.find(word).is_some_and(|node| node.is_end)
    }
    fn find(&self, word: String) -> Option<&TrieNode> {
        let mut node = &self.root;
        for c in word.bytes() {
            let index = (c - b'a') as usize;
            match node.children[index] {
                None => return None,
                Some(ref child) => node = child,
            }
        }
        Some(node)
    }

    fn starts_with(&self, prefix: String) -> bool {
        self.find(prefix).is_some()
    }
}

/// Your Trie object will be instantiated and called as such:
/// let obj = Trie::new();
/// obj.insert(word);
/// let ret_2: bool = obj.search(word);
/// let ret_3: bool = obj.starts_with(prefix);

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_208() {
        let mut trie = Trie::new();
        trie.insert("apple".to_string());
        assert_eq!(trie.search("apple".to_string()), true);
        assert_eq!(trie.search("app".to_string()), false);
        assert_eq!(trie.starts_with("app".to_string()), true);
        trie.insert("app".to_string());
        assert_eq!(trie.search("app".to_string()), true);
    }
    #[test]
    fn test_default_trie() {
        let trie = Trie::new();
        println!("{:?}", trie);
    }
    #[test]
    fn test_edge_cases() {
        let mut trie = Trie::new();

        // 空字符串处理
        trie.insert("".to_string());
        assert_eq!(trie.search("".to_string()), true);
        assert_eq!(trie.starts_with("".to_string()), true);

        // 单个字符测试
        trie.insert("a".to_string());
        assert_eq!(trie.search("a".to_string()), true);
        assert_eq!(trie.starts_with("a".to_string()), true);
    }

    #[test]
    fn test_performance() {
        let mut trie = Trie::new();
        let vec = b"abcdefghijklmnopqrstuvwxyz";

        // 大量插入测试
        for i in 0..1000 {
            trie.insert(format!("word{}", vec[i % vec.len()] as char));
        }

        // 大量查找测试
        for i in 0..1000 {
            assert_eq!(
                trie.search(format!("word{}", vec[i % vec.len()] as char)),
                true
            );
        }

        // 前缀查找测试
        assert_eq!(trie.starts_with("word".to_string()), true);
        assert_eq!(trie.starts_with("nonexistent".to_string()), false);
    }
}
