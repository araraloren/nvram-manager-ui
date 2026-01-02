## 统一字段命名实现计划

### 目标
- 确保NvramBackup及其相关结构体与AppConfig使用统一的命名规范
- Rust端使用蛇形命名，通过serde的rename属性序列化时转换为驼峰命名
- Vue端使用符合JavaScript规范的驼峰命名
- 保持前后端通信时字段名称一致

### 实现步骤

#### 1. 后端修改 - models.rs
- **PsVersion结构体**：为字段添加serde rename属性，转换为驼峰命名
- **JChipVersion结构体**：为字段添加serde rename属性，转换为驼峰命名
- **NvramBackup结构体**：为字段添加serde rename属性，转换为驼峰命名

#### 2. 前端修改 - types/index.ts
- **PsVersion接口**：保持不变（已使用驼峰命名）
- **JChipVersion接口**：保持不变（已使用驼峰命名）
- **NvramBackup接口**：将蛇形命名改为驼峰命名
- **NvramInfoWithPath接口**：保持不变

#### 3. 前端组件修改
- **App.vue**：
  - 更新currentNvramInfo的初始值，使用驼峰命名
  - 更新事件监听处理逻辑，使用驼峰命名
- **BackupList.vue**：
  - 更新模板中使用的字段名称，从蛇形命名改为驼峰命名
- **NvramInfo.vue**：
  - 更新模板中使用的字段名称，从蛇形命名改为驼峰命名

### 预期效果
- 后端Rust代码保持使用蛇形命名，符合Rust规范
- 前端Vue代码使用驼峰命名，符合JavaScript规范
- 通过serde rename属性实现前后端字段名称映射
- 统一的命名规范提高代码的可读性和维护性

### 修改的文件
1. `src-tauri/src/modules/models.rs` - 后端结构体定义
2. `src/types/index.ts` - 前端类型定义
3. `src/App.vue` - 主应用组件
4. `src/components/BackupList.vue` - 备份列表组件
5. `src/components/NvramInfo.vue` - NVRAM信息组件

### 实现细节
- 所有字段将遵循驼峰命名规范（如nvramName, psVersion, jchipVersion等）
- 确保嵌套结构体（如PsVersion, JChipVersion）也使用统一的命名规范
- 保持字段语义不变，只修改命名格式