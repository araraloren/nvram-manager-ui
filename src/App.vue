<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen, UnlistenFn } from "@tauri-apps/api/event";

// 导入组件
import NvramInfo from './components/NvramInfo.vue';
import BackupList from './components/BackupList.vue';
import StatusBar from './components/StatusBar.vue';

// 导入类型定义
import type { StatusUpdate, NvramBackup, NvramInfoWithPath, AppConfig } from './types/index';

// 主题相关
const isDarkTheme = ref(false);

// 监听主题变化
watch(isDarkTheme, () => {
  updateTheme();
});

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

// 初始化主题
const initTheme = () => {
  // 从本地存储获取主题偏好
  const savedTheme = localStorage.getItem('theme');
  if (savedTheme) {
    isDarkTheme.value = savedTheme === 'dark';
  } else {
    // 默认使用系统主题
    isDarkTheme.value = window.matchMedia('(prefers-color-scheme: dark)').matches;
  }
  updateTheme();
};

// 更新主题
const updateTheme = () => {
  if (isDarkTheme.value) {
    document.body.classList.add('dark-theme');
    localStorage.setItem('theme', 'dark');
  } else {
    document.body.classList.remove('dark-theme');
    localStorage.setItem('theme', 'light');
  }
};

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
  initTheme();
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
      
      <!-- 主题切换开关 -->
      <div class="theme-toggle">
        <span class="theme-toggle-label">{{ isDarkTheme ? '🌙 深色' : '☀️ 浅色' }}</span>
        <label class="toggle-switch">
          <input type="checkbox" v-model="isDarkTheme">
          <span class="toggle-slider"></span>
        </label>
      </div>
      
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