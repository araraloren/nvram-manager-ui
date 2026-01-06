<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

// 导入类型定义
import type { NvramBackup, AppConfig } from '../types/index';

// 接收props
const props = defineProps<{
  currentNvramInfo: NvramBackup;
  appConfig: AppConfig;
}>();

// 开关状态
const clearAfterBackup = ref(props.appConfig.clearAfterBackup);

// 执行备份操作
const handleBackup = async () => {
  try {
    const result = await invoke<boolean>("backup_nvram", {
      clearAfter: clearAfterBackup.value
    });
    if (result) {
      console.log("备份成功");
      // 备份成功后，父组件会通过事件监听更新备份列表
    }
  } catch (error) {
    console.error("备份失败:", error);
  }
};
</script>

<template>
  <div class="info-card">
    <h2 class="section-title">NVRAM信息</h2>
    
    <div class="info-grid">
      <!-- 1. NVRAM路径 -->
      <div class="info-item full-width">
        <div class="info-icon path-icon">📁</div>
        <div class="info-content">
          <div class="info-label">PS路径</div>
          <div class="info-value">{{ currentNvramInfo.psVersion.path }}</div>
        </div>
      </div>
      
      <!-- 2. 游戏路径 -->
      <div class="info-item full-width">
        <div class="info-icon path-icon">🎮</div>
        <div class="info-content">
          <div class="info-label">游戏路径</div>
          <div class="info-value">{{ currentNvramInfo.gamePath }}</div>
        </div>
      </div>
      
      <!-- 3. NVRAM名称 -->
      <div class="info-item">
        <div class="info-icon game-icon">🎮</div>
        <div class="info-content">
          <div class="info-label">NVRAM名称</div>
          <div class="info-value">{{ currentNvramInfo.nvramName }}</div>
        </div>
      </div>
      
      <!-- 4. 文件数量 -->
      <div class="info-item">
        <div class="info-icon file-icon">📄</div>
        <div class="info-content">
          <div class="info-label">文件数量</div>
          <div class="info-value">{{ currentNvramInfo.fileList.length }}</div>
        </div>
      </div>
      
      <!-- 5. PS版本 -->
      <div class="info-item">
        <div class="info-icon ps-icon">🔧</div>
        <div class="info-content">
          <div class="info-label">依赖PS版本</div>
          <div class="info-value">{{ currentNvramInfo.psVersion.version }}</div>
        </div>
      </div>
      
      <!-- 6. JChip版本 -->
      <div class="info-item">
        <div class="info-icon jchip-icon">🔧</div>
        <div class="info-content">
          <div class="info-label">依赖JChip版本</div>
          <div class="info-value">{{ currentNvramInfo.jchipVersion.version }}</div>
        </div>
      </div>
    </div>
    
    <!-- 备份操作 -->
    <div class="backup-operation">
      <h3 class="subsection-title">备份操作</h3>
      
      <div class="backup-content">
        <!-- 备份按钮 -->
        <div class="backup-button-container">
          <button class="backup-button" @click="handleBackup">
            <span class="backup-icon">💾</span>
            <span class="backup-text">备份NVRAM</span>
          </button>
        </div>
        
        <!-- 开关选项 -->
        <div class="backup-options">
          <!-- 备份后清除开关 -->
          <div class="option-item">
            <div class="option-content">
              <div class="option-icon clear-icon">🗑️</div>
              <div class="option-label">备份后清除当前NVRAM</div>
            </div>
            <label class="toggle-switch">
              <input type="checkbox" v-model="clearAfterBackup">
              <span class="toggle-slider"></span>
            </label>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
