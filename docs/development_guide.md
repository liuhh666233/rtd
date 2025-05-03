# RTD (Rust Todo) 开发流程文档

## 1. 项目初始化

```bash
# 创建新项目
cargo new rtd
cd rtd

# 添加依赖到 Cargo.toml
[package]
name = "rtd"
version = "0.1.0"
edition = "2021"

[dependencies]
clap = { version = "4.0", features = ["derive"] }
chrono = "0.4"
```

## 2. 数据模型层 (model.rs)

### 2.1 核心数据结构
```rust
pub struct Item {
    pub(crate) id: u32,           // 待办事项ID
    pub(crate) name: String,      // 待办事项名称
    pub(crate) completed: bool,   // 完成状态
    pub(crate) deleted: bool,     // 删除状态
    pub(crate) created_at: Option<i64>,    // 创建时间
    pub(crate) completed_at: Option<i64>,  // 完成时间
    pub(crate) deleted_at: Option<i64>,    // 删除时间
}
```

### 2.2 Item 实现的方法
```rust
impl Item {
    // 创建新的待办事项
    pub fn new(
        id: u32,
        name: &str,
        completed: bool,
        deleted: bool,
        created_at: Option<i64>,
        completed_at: Option<i64>,
        deleted_at: Option<i64>,
    ) -> Self

    // 获取ID
    pub fn id(&self) -> u32

    // 格式化输出
    pub fn to_prettier_string(&self) -> String
}
```

### 2.3 序列化实现
```rust
// 序列化为CSV格式
impl ToString for Item {
    fn to_string(&self) -> String
}

// 从CSV格式反序列化
impl FromStr for Item {
    type Err = ParseItemError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err>
}
```

## 3. 存储层 (storage.rs)

### 3.1 CSV文件操作
```rust
struct Csv {
    filename: String,
    file: File,
}

impl Csv {
    // 创建/打开CSV文件
    fn new() -> Result<Self>
    
    // 创建新文件
    fn create(path: &Path) -> Result<fs::File>
    
    // 打开已存在文件
    fn open(path: &Path) -> Result<fs::File>
    
    // 获取文件内容
    fn content(&mut self) -> Result<String>
    
    // 在指定位置修改内容
    fn splice(&mut self, offset: u64, delete_size: u64, write_content: &str) -> Result<()>
}
```

### 3.2 存储操作接口
```rust
// 添加待办事项
pub fn add_item(item: Item) -> Result<()>

// 更新待办事项
pub fn update_item(item: Item) -> Result<()>

// 删除待办事项
pub fn delete_item(id: u32) -> Result<()>

// 获取所有待办事项
pub fn get_all() -> Result<Vec<Item>>

// 根据ID获取待办事项
pub fn get_item_by_id(id: u32) -> Result<Item>

// 获取最大ID
pub fn get_max_id() -> Result<u32>
```

## 4. 业务逻辑层 (service.rs)

### 4.1 核心业务功能
```rust
// 添加新的待办事项
pub fn add_item(name: &str) -> Result<String>

// 完成待办事项
pub fn complete_item(id: u32) -> Result<String>

// 取消完成待办事项
pub fn uncomplete_item(id: u32) -> Result<String>

// 删除待办事项（软删除）
pub fn delete_item(id: u32) -> Result<String>

// 恢复已删除的待办事项
pub fn restore_item(id: u32) -> Result<String>

// 销毁已删除的待办事项
pub fn destroy_deleted() -> Result<String>

// 销毁指定待办事项
pub fn destroy_item(id: u32) -> Result<String>

// 清空所有待办事项
pub fn clear() -> Result<String>
```

### 4.2 列表展示功能
```rust
// 显示未完成的待办事项
pub fn list_uncompleted() -> Result<String>

// 显示已完成的待办事项
pub fn list_completed() -> Result<String>

// 显示已删除的待办事项
pub fn list_deleted() -> Result<String>

// 显示所有待办事项
pub fn list_all() -> Result<String>
```

## 5. 命令行接口 (main.rs)

### 5.1 命令行参数定义
```rust
#[derive(Parser, Debug)]
struct Args {
    // 添加待办事项
    #[arg(short, long, value_name = "item-name")]
    add: Option<String>,

    // 完成待办事项
    #[arg(short, long, value_name = "item-id")]
    complete: Option<u32>,

    // 取消完成待办事项
    #[arg(short, long, value_name = "item-id")]
    uncomplete: Option<u32>,

    // 删除待办事项
    #[arg(short, long, value_name = "item-id")]
    delete: Option<u32>,

    // 恢复待办事项
    #[arg(short, long, value_name = "item-id")]
    restore: Option<u32>,

    // 销毁待办事项
    #[arg(long, value_name = "item-id")]
    destroy: Option<u32>,

    // 销毁所有已删除的待办事项
    #[arg(long)]
    destroy_deleted: bool,

    // 清空所有记录
    #[arg(long)]
    clear: bool,

    // 列表显示
    #[arg(short, long, value_name = "list-type")]
    list: Option<Option<ListType>>,
}
```

### 5.2 列表类型枚举
```rust
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum ListType {
    All,         // 所有待办事项
    Completed,   // 已完成的待办事项
    Uncompleted, // 未完成的待办事项
    Deleted,     // 已删除的待办事项
}
```

## 6. 错误处理

### 6.1 Model层错误
```rust
#[derive(Debug)]
pub struct ParseItemError(String);
```

### 6.2 Storage层错误
```rust
#[derive(Debug)]
pub enum StorageError {
    FileHandle(FileHandleError),
    ParseItem(ParseItemError),
    ItemNoExist(u32),
}
```

### 6.3 Service层错误
```rust
#[derive(Debug)]
pub enum ServiceError {
    Storage(StorageError),
}
```

## 7. 使用示例

```bash
# 添加待办事项
rtd --add "完成RTD项目开发"

# 完成待办事项
rtd --complete 1

# 列出所有未完成的待办事项
rtd --list

# 删除待办事项
rtd --delete 1

# 恢复已删除的待办事项
rtd --restore 1

# 销毁已删除的待办事项
rtd --destroy-deleted

# 清空所有待办事项
rtd --clear
```

## 8. 数据存储格式

CSV文件格式 (.rtd.csv):
```csv
id,name,completed,deleted,createdAt,completedAt,deletedAt
1,完成RTD项目开发,false,false,1648888888,,
2,学习Rust编程,true,false,1648888889,1648888890,
``` 