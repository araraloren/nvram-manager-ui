## 修改NvramBackup结构体（前后端）

### 1. 修改后端代码 (Rust)

#### 1.1 修改models.rs
- 从`NvramBackup`结构体中移除`total_size`字段
- 添加`game_path`字段，类型为`PathBuf`，使用serde重命名为`gamePath`

#### 1.2 修改backup.rs
- 移除对`calculate_total_size`函数的导入
- 移除第54行和第104行的`total_size`计算
- 移除第64行和第114行的`total_size`字段赋值
- 在`generate_backups`函数中为`game_path`添加模拟路径
- 在`get_current_nvram_info`函数中为`game_path`添加模拟路径

#### 1.3 保留utils.rs中的calculate_total_size函数
- 该函数可能在其他地方被使用，或者可以保留以备将来使用

### 2. 修改前端代码 (TypeScript)

#### 2.1 修改src/types/index.ts
- 从`NvramBackup`接口中移除`totalSize`字段
- 添加`gamePath`字段，类型为`string`
- 确保`NvramInfoWithPath`接口也能正确继承新的结构

### 修改后效果
- 前后端的NvramBackup结构将保持一致
- 不再包含total_size字段
- 新增game_path/gamePath字段用于存储游戏路径
- 所有创建NvramBackup实例的地方都将添加game_path字段
- 移除了不必要的文件大小计算逻辑
- 前端类型定义与后端结构体保持同步

### 修改步骤
1. 先修改后端models.rs定义
2. 修改后端backup.rs实现
3. 最后修改前端类型定义
4. 确保所有修改保持一致性