<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen, UnlistenFn } from "@tauri-apps/api/event";

// 导入组件
import NvramInfo from './components/NvramInfo.vue';
import BackupList from './components/BackupList.vue';
import StatusBar from './components/StatusBar.vue';

// 导入类型定义
import type { StatusUpdate, NvramBackup, NvramInfoWithPath, AppConfig } from './types/index';

// 配置数据
const appConfig = ref<AppConfig>({
  nvramPath: "",
  backupPath: "",
  forceBackup: true,
  clearAfterBackup: false,
  clearNvramOnRestore: true,
  clearBackupOnRestore: false
});

// 当前NVRAM信息（使用NvramBackup类型）
const currentNvramInfo = ref<NvramBackup>({
  id: 0,
  nvramName: "",
  psVersion: {
    version: "",
    build: "",
    path: ""
  },
  jchipVersion: {
    version: "",
    build: "",
    path: ""
  },
  backupTime: "",
  fileList: [],
  totalSize: ""
});

// 备份数据
const backups = ref<NvramBackup[]>([]);

// 选中的备份
const selectedBackup = ref<NvramBackup | null>(null);

// 状态信息响应式变量
const statusInfo = ref<StatusUpdate>({
  level: 'info',
  message: '就绪'
});

// 事件监听器引用
let nvramInfoListener: UnlistenFn | null = null;
let statusUpdateListener: UnlistenFn | null = null;

// 从后端获取配置
const fetchAppConfig = async () => {
  try {
    const config = await invoke<AppConfig>("get_app_config");
    appConfig.value = config;
  } catch (error) {
    console.error("获取配置失败:", error);
  }
};

// 从后端获取当前NVRAM信息
const fetchCurrentNvramInfo = async () => {
  try {
    const info = await invoke<NvramBackup>("get_current_nvram_info_command");
    currentNvramInfo.value = info;
  } catch (error) {
    console.error("获取当前NVRAM信息失败:", error);
  }
};

// 从后端获取备份列表
const fetchBackupList = async () => {
  try {
    const backupList = await invoke<NvramBackup[]>("get_backup_list");
    backups.value = backupList;
  } catch (error) {
    console.error("获取备份列表失败:", error);
  }
};

// 设置NVRAM信息变化监听器
const setupNvramInfoListener = async () => {
  if (nvramInfoListener) return;
  
  nvramInfoListener = await listen<NvramInfoWithPath>("nvram_info_updated", (event) => {
    console.log("收到NVRAM信息更新事件:", event.payload);
    // 更新当前NVRAM信息
    // 从事件中提取NvramBackup需要的字段
    const { id, nvramName, psVersion, jchipVersion, fileList, totalSize } = event.payload;
    currentNvramInfo.value = {
      id,
      nvramName,
      psVersion,
      jchipVersion,
      backupTime: event.payload.backupTime || "",
      fileList,
      totalSize
    };
  });
};

// 设置状态更新监听器
const setupStatusUpdateListener = async () => {
  if (statusUpdateListener) return;
  
  statusUpdateListener = await listen<StatusUpdate>("status_updated", (event) => {
    console.log("收到状态更新事件:", event.payload);
    // 更新状态信息
    statusInfo.value = event.payload;
  });
};

// 移除NVRAM信息变化监听器
const removeNvramInfoListener = () => {
  if (nvramInfoListener) {
    nvramInfoListener();
    nvramInfoListener = null;
  }
};

// 移除状态更新监听器
const removeStatusUpdateListener = () => {
  if (statusUpdateListener) {
    statusUpdateListener();
    statusUpdateListener = null;
  }
};

// 更新选中的备份
const updateSelectedBackup = (backup: any) => {
  selectedBackup.value = backup;
};

onMounted(async () => {
  fetchAppConfig();
  fetchCurrentNvramInfo();
  fetchBackupList();
  await setupNvramInfoListener();
  await setupStatusUpdateListener();
});

onUnmounted(() => {
  removeNvramInfoListener();
  removeStatusUpdateListener();
});
</script>

<template>
  <div class="app-container">
    <!-- 程序显示区域 -->
    <main class="main-content">
      <h1>欢迎使用NVRAM管理器</h1>
      
      <!-- 左右分栏布局 -->
      <div class="main-layout">
        <!-- 左侧栏 -->
        <div class="left-panel">
          <!-- 信息显示区域 -->
          <div class="info-section">
            <NvramInfo 
              :current-nvram-info="currentNvramInfo" 
              :app-config="appConfig" 
            />
          </div>
        </div>
        
        <!-- 右侧栏 -->
        <div class="right-panel">
          <BackupList 
            :backups="backups" 
            :app-config="appConfig"
            @update-selected-backup="updateSelectedBackup"
          />
        </div>
      </div>
    </main>
    
    <!-- 状态栏组件 -->
    <StatusBar 
      :backup-path="appConfig.backupPath" 
      :status-info="statusInfo" 
    />
  </div>
</template>

<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

html, body, #app {
  width: 100%;
  height: 100%;
  overflow: hidden;
}

.app-container {
  display: flex;
  flex-direction: column;
  width: 100%;
  height: 100%;
  background-color: #f6f6f6;
}

.main-content {
  flex: 1;
  padding: 20px;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  background-color: #ffffff;
  margin: 10px;
  border-radius: 8px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.main-content h1 {
  margin-bottom: 20px;
  color: #333333;
  width: 100%;
}

/* 左右分栏布局 */
.main-layout {
  display: flex;
  gap: 20px;
  width: 100%;
  height: calc(100% - 60px); /* 减去标题和间距 */
  overflow: hidden;
  align-items: flex-start;
}

.left-panel {
  flex: 0 0 30%;
  display: flex;
  flex-direction: column;
  gap: 20px;
  overflow-y: auto;
  overflow-x: hidden;
  height: 100%;
}

.right-panel {
  flex: 0 0 70%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  height: 100%;
}

/* 信息显示区域 */
.info-section {
  width: 100%;
}

.info-card {
  background-color: #fafafa;
  border-radius: 8px;
  padding: 20px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  height: 100%;
}

.section-title {
  font-size: 18px;
  font-weight: 600;
  margin-bottom: 15px;
  color: #444444;
  border-bottom: 1px solid #e0e0e0;
  padding-bottom: 8px;
}

/* 子标题样式 */
.subsection-title {
  font-size: 16px;
  font-weight: 600;
  margin: 20px 0 15px 0;
  color: #555555;
  border-bottom: 1px solid #e0e0e0;
  padding-bottom: 8px;
}

.info-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
  gap: 15px;
}

/* 备份操作区域 */
.backup-operation {
  width: 100%;
  margin-top: 20px;
}

/* 调整备份内容的间距 */
.backup-operation .backup-content {
  display: flex;
  flex-direction: column;
  gap: 15px;
}

.info-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px;
  background-color: #ffffff;
  border-radius: 6px;
  border: 1px solid #e0e0e0;
  transition: all 0.2s ease;
}

.info-item:hover {
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.15);
  transform: translateY(-1px);
}

.info-icon {
  font-size: 24px;
  width: 40px;
  text-align: center;
}

.info-content {
  flex: 1;
}

.info-label {
  font-size: 12px;
  color: #666666;
  margin-bottom: 2px;
}

.info-value {
  font-size: 14px;
  font-weight: 500;
  color: #333333;
}

/* 备份按钮 */
.backup-button-container {
  display: flex;
  justify-content: flex-start;
}

.backup-button {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px 24px;
  background-color: #4f46e5;
  color: white;
  border: none;
  border-radius: 6px;
  font-size: 16px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.backup-button:hover {
  background-color: #4338ca;
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
  transform: translateY(-1px);
}

.backup-button:active {
  background-color: #3730a3;
  transform: translateY(0);
}

.backup-icon {
  font-size: 18px;
}

/* 备份选项 */
.backup-options {
  display: flex;
  flex-direction: column;
  gap: 15px;
}

.option-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px;
  background-color: #ffffff;
  border-radius: 6px;
  border: 1px solid #e0e0e0;
}

.option-content {
  display: flex;
  align-items: center;
  gap: 12px;
}

.option-icon {
  font-size: 20px;
  width: 30px;
  text-align: center;
}

.option-label {
  font-size: 14px;
  color: #333333;
  font-weight: 500;
}

/* 开关样式 */
.toggle-switch {
  position: relative;
  display: inline-block;
  width: 50px;
  height: 24px;
}

.toggle-switch input {
  opacity: 0;
  width: 0;
  height: 0;
}

.toggle-slider {
  position: absolute;
  cursor: pointer;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: #ccc;
  transition: .4s;
  border-radius: 24px;
}

.toggle-slider:before {
  position: absolute;
  content: "";
  height: 18px;
  width: 18px;
  left: 3px;
  bottom: 3px;
  background-color: white;
  transition: .4s;
  border-radius: 50%;
}

input:checked + .toggle-slider {
  background-color: #4f46e5;
}

input:focus + .toggle-slider {
  box-shadow: 0 0 1px #4f46e5;
}

input:checked + .toggle-slider:before {
  transform: translateX(26px);
}

/* 备份信息展示区域 */
.backup-info-section {
  width: 100%;
  display: flex;
  flex-direction: row;
  gap: 20px;
  flex: 1 1 auto;
  min-height: 0;
  height: 100%;
}

.backup-list {
  flex: 0 0 40%;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  min-height: 0;
  height: 100%;
}

.list-title {
  font-size: 16px;
  font-weight: 600;
  margin-bottom: 10px;
  color: #555555;
}

.backup-list .info-card {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.backup-list-container {
  background-color: #ffffff;
  border-radius: 6px;
  border: 1px solid #e0e0e0;
  overflow: auto;
  flex: 1 1 auto;
  min-height: 0;
}

.backup-item {
  padding: 15px;
  border-bottom: 1px solid #e0e0e0;
  cursor: pointer;
  transition: all 0.2s ease;
}

.backup-item:last-child {
  border-bottom: none;
}

.backup-item:hover {
  background-color: #f5f5f5;
}

.backup-item-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 5px;
}

.backup-item-name {
  font-weight: 600;
  color: #333333;
  font-size: 14px;
}

.backup-item-status {
  font-size: 12px;
  color: #22c55e;
  font-weight: 500;
}

.backup-item-meta {
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-size: 12px;
  color: #666666;
}

.backup-item-date {
  flex: 1;
}

.backup-item-size {
  flex: 1;
  text-align: right;
}

/* 备份详情容器 */
.backup-detail-container {
  flex: 0 0 55%;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  height: 100%;
}

/* 备份详情 */
.backup-detail {
  width: 100%;
  margin-top: 0;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  height: 100%;
}

/* 备份详情占位符 */
.backup-detail-placeholder {
  width: 100%;
  margin-top: 0;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  height: 100%;
}

.backup-detail .info-card,
.backup-detail-placeholder .info-card {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.backup-detail-content {
  background-color: #ffffff;
  border-radius: 6px;
  border: 1px solid #e0e0e0;
  padding: 15px;
  overflow: auto;
  flex: 1 1 auto;
  min-height: 0;
}

/* 占位符内容 */
.placeholder-content {
  background-color: #ffffff;
  border-radius: 6px;
  border: 1px solid #e0e0e0;
  padding: 15px;
  text-align: center;
  color: #999999;
  flex: 1 1 auto;
  min-height: 0;
  display: flex;
  align-items: center;
  justify-content: center;
}

.detail-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 10px 0;
  border-bottom: 1px solid #f0f0f0;
}

.detail-item:last-child {
  border-bottom: none;
}

.detail-label {
  font-size: 13px;
  color: #666666;
  font-weight: 500;
}

.detail-value {
  font-size: 13px;
  color: #333333;
  font-weight: 500;
}

/* 还原选项 */
.restore-options {
  margin: 20px 0;
  padding: 15px;
  background-color: #f5f5f5;
  border-radius: 6px;
  border: 1px solid #e0e0e0;
}

.options-title {
  font-size: 14px;
  font-weight: 600;
  margin-bottom: 15px;
  color: #555555;
}

.restore-option-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 10px 0;
  border-bottom: 1px solid #e0e0e0;
}

.restore-option-item:last-child {
  border-bottom: none;
}

/* 还原按钮 */
.restore-button-container {
  display: flex;
  gap: 12px;
  justify-content: flex-start;
  margin-top: 20px;
}

.restore-button {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 20px;
  background-color: #10b981;
  color: white;
  border: none;
  border-radius: 6px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.restore-button:hover {
  background-color: #059669;
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
  transform: translateY(-1px);
}

.restore-button:active {
  background-color: #047857;
  transform: translateY(0);
}

.restore-icon {
  font-size: 16px;
}

/* 删除按钮 */
.delete-button {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 20px;
  background-color: #ef4444;
  color: white;
  border: none;
  border-radius: 6px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.delete-button:hover {
  background-color: #dc2626;
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
  transform: translateY(-1px);
}

.delete-button:active {
  background-color: #b91c1c;
  transform: translateY(0);
}

.delete-icon {
  font-size: 16px;
}

/* 确认对话框样式 */
.confirm-dialog-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.confirm-dialog {
  background-color: white;
  border-radius: 8px;
  padding: 24px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  max-width: 400px;
  width: 90%;
}

.confirm-dialog-title {
  font-size: 18px;
  font-weight: 600;
  margin-bottom: 12px;
  color: #333;
}

.confirm-dialog-message {
  font-size: 14px;
  color: #666;
  margin-bottom: 24px;
  line-height: 1.5;
}

.confirm-dialog-buttons {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
}

.confirm-dialog-cancel {
  padding: 8px 16px;
  background-color: #f0f0f0;
  color: #333;
  border: 1px solid #e0e0e0;
  border-radius: 6px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
}

.confirm-dialog-cancel:hover {
  background-color: #e0e0e0;
  border-color: #d0d0d0;
}

.confirm-dialog-delete {
  padding: 8px 16px;
  background-color: #ef4444;
  color: white;
  border: none;
  border-radius: 6px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
}

.confirm-dialog-delete:hover {
  background-color: #dc2626;
}

/* 状态栏 */
.status-bar {
  height: 30px;
  background-color: #e0e0e0;
  border-top: 1px solid #d0d0d0;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 15px;
  font-size: 12px;
  color: #666666;
}

.status-left {
  text-align: left;
  max-width: 50%;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.status-right {
  text-align: right;
}

/* 状态信息样式 */
.status-message {
  font-weight: 500;
}

/* 不同状态级别的颜色 */
.status-message.info {
  color: #10b981; /* 绿色 - 提示 */
}

.status-message.warning {
  color: #f59e0b; /* 黄色 - 警告 */
}

.status-message.error {
  color: #ef4444; /* 红色 - 错误 */
}

/* 深色主题支持 */
@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  .app-container {
    background-color: #2f2f2f;
  }

  .main-content {
    background-color: #3a3a3a;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
  }

  .main-content h1 {
    color: #f6f6f6;
  }

  .info-card {
    background-color: #444444;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.3);
  }

  .section-title {
    color: #e0e0e0;
    border-bottom: 1px solid #555555;
  }

  .info-item {
    background-color: #4a4a4a;
    border: 1px solid #555555;
  }

  .info-item:hover {
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.4);
  }

  .info-label {
    color: #aaaaaa;
  }

  .info-value {
    color: #f6f6f6;
  }

  /* 备份操作深色主题 */
  .backup-button {
    background-color: #6366f1;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
  }

  .backup-button:hover {
    background-color: #4f46e5;
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.4);
  }

  .backup-button:active {
    background-color: #4338ca;
  }

  .option-item {
    background-color: #4a4a4a;
    border: 1px solid #555555;
  }

  .option-label {
    color: #e0e0e0;
  }

  .toggle-slider {
    background-color: #555555;
  }

  .toggle-slider:before {
    background-color: #f6f6f6;
  }

  input:checked + .toggle-slider {
    background-color: #6366f1;
  }

  /* 备份信息深色主题 */
  .list-title {
    color: #e0e0e0;
  }

  .backup-list-container {
    background-color: #4a4a4a;
    border: 1px solid #555555;
  }

  .backup-item {
    border-bottom: 1px solid #555555;
  }

  .backup-item:hover {
    background-color: #505050;
  }

  .backup-item-name {
    color: #e0e0e0;
  }

  .backup-item-status {
    color: #34d399;
  }

  .backup-item-meta {
    color: #aaaaaa;
  }

  .backup-detail-content {
    background-color: #4a4a4a;
    border: 1px solid #555555;
  }

  .detail-item {
    border-bottom: 1px solid #555555;
  }

  .detail-label {
    color: #aaaaaa;
  }

  .detail-value {
    color: #e0e0e0;
  }

  /* 还原选项深色主题 */
  .restore-options {
    background-color: #454545;
    border: 1px solid #555555;
  }

  .options-title {
    color: #e0e0e0;
  }

  .restore-option-item {
    border-bottom: 1px solid #555555;
  }

  .restore-button {
    background-color: #059669;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
  }

  .restore-button:hover {
    background-color: #047857;
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.4);
  }

  .restore-button:active {
    background-color: #065f46;
  }

  /* 删除按钮深色主题 */
  .delete-button {
    background-color: #dc2626;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
  }

  .delete-button:hover {
    background-color: #b91c1c;
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.4);
  }

  .delete-button:active {
    background-color: #991b1b;
  }

  /* 确认对话框深色主题 */
  .confirm-dialog {
    background-color: #444444;
  }

  .confirm-dialog-title {
    color: #e0e0e0;
  }

  .confirm-dialog-message {
    color: #cccccc;
  }

  .confirm-dialog-cancel {
    background-color: #555555;
    color: #e0e0e0;
    border-color: #666666;
  }

  .confirm-dialog-cancel:hover {
    background-color: #666666;
    border-color: #777777;
  }

  .confirm-dialog-delete {
    background-color: #dc2626;
  }

  .confirm-dialog-delete:hover {
    background-color: #b91c1c;
  }

  /* 状态栏深色主题 */
  .status-bar {
    background-color: #2a2a2a;
    border-top: 1px solid #1a1a1a;
    color: #cccccc;
  }

  /* 状态信息深色主题颜色 */
  .status-message.info {
    color: #34d399; /* 绿色 - 提示 */
  }

  .status-message.warning {
    color: #fbbf24; /* 黄色 - 警告 */
  }

  .status-message.error {
    color: #ef4444; /* 红色 - 错误 */
  }
}
</style>
