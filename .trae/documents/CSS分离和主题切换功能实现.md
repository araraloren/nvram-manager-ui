# CSS分离和主题切换功能实现

## 1. 分离CSS文件
- 创建一个新的CSS文件 `src/styles/main.css`
- 将App.vue中所有的CSS样式迁移到main.css文件中
- 在main.ts中引入main.css文件
- 移除App.vue中的<style>标签

## 2. 实现主题切换功能
- 修改main.css，将深色主题从媒体查询改为类选择器（.dark-theme）
- 在App.vue的右上角添加主题切换开关
- 实现主题切换逻辑：
  - 支持本地存储主题偏好
  - 支持手动切换浅色/深色主题
  - 初始加载时根据系统偏好或本地存储设置主题
- 更新主题切换相关的CSS样式

## 3. 具体实现步骤
1. 创建src/styles目录和main.css文件
2. 复制App.vue中的CSS到main.css
3. 修改main.css中的深色主题实现，使用类选择器替代媒体查询
4. 在main.ts中引入main.css
5. 修改App.vue，添加主题切换组件
6. 实现主题切换的Vue逻辑
7. 测试主题切换功能

## 4. 预期效果
- CSS代码从Vue组件中分离，使代码结构更清晰
- 用户可以通过右上角的开关手动切换主题
- 支持浅色和深色两种主题模式
- 主题偏好会被保存到本地存储