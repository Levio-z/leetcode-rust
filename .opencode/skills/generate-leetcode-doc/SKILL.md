---
name: generate-leetcode-doc
description: Generate markdown documentation for LeetCode Rust solutions following project conventions
license: MIT
compatibility: opencode
metadata:
  audience: developers
  workflow: documentation
---

# Generate LeetCode Documentation

You are a documentation generator for LeetCode Rust solutions. When asked to generate documentation:

## Input

- LeetCode problem details (title, description, examples, constraints)
- Rust solution code file path

## Output Format

Generate a markdown file in `doc/` directory with naming convention:

- Format: `s{题号4位}_{序号2位}_{中文标题}.md`
- Example: `s0001_01_哈希表.md`, `s0242_01_有效的字母异位词.md`

## Documentation Structure

```markdown
# 题号. 标题（副标题）

## 问题描述

[问题简要描述]

### 示例

- **示例 1**：
  - 输入：`...`
  - 输出：`...`

### 约束条件

- 约束1
- 约束2

### 进阶问题（可选）

[进阶要求描述]

## 实现思路

### 方法概述

[核心思路1-2句话]

### 代码实现

```rust
[提取 impl Solution 中的 pub fn 实现]
```

## 实现分析

### 核心思路

1. 步骤一描述
2. 步骤二描述
3. 步骤三描述

### 代码细节

- 关键点1
- 关键点2
- 关键点3

### 性能分析

| 类型 | 复杂度 | 说明 |
|------|--------|------|
| 时间复杂度 | O(n) | 理由 |
| 空间复杂度 | O(1) | 理由 |

## 测试用例

```rust
[从代码中提取 #[cfg(test)] mod tests 中的测试]
```

## 扩展思考

### 进阶问题解答

[针对进阶要求的解决方案]

### 其他可能的实现方式

1. **方式一**：
   - 思路描述
   - 复杂度分析

2. **方式二**：
   - 思路描述
   - 复杂度分析

## 总结

1. 方法适用场景
2. 核心技巧
3. 选择建议
```

## Key Points

- Use Chinese headings and descriptions
- Include code comments from the Rust source (especially // 注释)
- Preserve all code examples with proper Rust syntax highlighting
- Match variable naming and comments from source code
- Output to doc/ directory with proper filename
- If multiple solution methods exist (e.g., is_anagram, is_anagram2), document both
- Include performance analysis table
- Extract test cases from #[cfg(test)] module

## Example Usage

- Generate doc for `src/solution/s0001_two_sum.rs` with title "哈希表优化"
- Generate doc for `src/solution/s0242_valid_anagram.rs` with title "有效的字母异位词"
