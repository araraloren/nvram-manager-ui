<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

// 导入类型定义
import type { NvramBackup, AppConfig } from '../types/index';

// 接收props
const props = defineProps<{
  backups: NvramBackup[];
  appConfig: AppConfig;
}>();

// 定义事件
const emit = defineEmits<{
  updateSelectedBackup: [backup: NvramBackup | null];
}>();

// 选中的备份
const selectedBackup = ref<NvramBackup | null>(null);

// 还原开关状态
const clearNvramOnRestore = ref(props.appConfig.clearNvramOnRestore);
const clearBackupOnRestore = ref(props.appConfig.clearBackupOnRestore);

// 确认对话框状态
const showConfirmDialog = ref(false);

// 选择备份
const selectBackup = (backup: NvramBackup) => {
  selectedBackup.value = backup;
  emit('updateSelectedBackup', backup);
};

// 执行还原操作
const handleRestore = async () => {
  if (!selectedBackup.value) return;
  
  try {
    const result = await invoke<boolean>("restore_backup", {
      backupId: selectedBackup.value.id,
      clearNvram: clearNvramOnRestore.value,
      clearBackup: clearBackupOnRestore.value
    });
    if (result) {
      console.log("还原成功");
      // 还原成功后，父组件会通过事件监听更新备份列表
    }
  } catch (error) {
    console.error("还原失败:", error);
  }
};

// 显示确认对话框
const handleShowConfirm = () => {
  if (selectedBackup.value) {
    showConfirmDialog.value = true;
  }
};

// 确认删除
const handleConfirmDelete = async () => {
  if (!selectedBackup.value) return;
  
  try {
    const result = await invoke<boolean>("delete_backup", {
      backupId: selectedBackup.value.id
    });
    if (result) {
      console.log("删除成功");
      // 删除成功后，通知父组件更新备份列表
      emit('updateSelectedBackup', null);
    }
  } catch (error) {
    console.error("删除失败:", error);
  } finally {
    // 关闭确认对话框
    showConfirmDialog.value = false;
  }
};

// 取消删除
const handleCancelDelete = () => {
  showConfirmDialog.value = false;
};
</script>

<template>
  <!-- 备份信息展示区域 -->
  <div class="backup-info-section">
    <!-- 备份列表 -->
    <div class="backup-list">
      <div class="info-card">
        <h3 class="list-title">备份列表</h3>
        <div class="backup-list-container">
          <div 
            v-for="(backup, index) in backups" 
            :key="index"
            class="backup-item"
            @click="selectBackup(backup)"
          >
            <div class="backup-item-header">
              <div class="backup-item-name">{{ backup.nvramName }}_{{ backup.psVersion.version }}_{{ backup.jchipVersion.version }}</div>
              <div class="backup-item-status">完成</div>
            </div>
            <div class="backup-item-meta">
              <div class="backup-item-date">{{ new Date(backup.backupTime).toLocaleString() }}</div>
              <div class="backup-item-size">{{ backup.totalSize }} ({{ backup.fileList.length }} files)</div>
            </div>
          </div>
        </div>
      </div>
    </div>
    
    <!-- 备份详情容器 -->
    <div class="backup-detail-container">
      <!-- 备份详情 -->
      <div v-if="selectedBackup" class="backup-detail">
        <div class="info-card">
          <h3 class="list-title">备份详情</h3>
          <div class="backup-detail-content">
            <div class="detail-item">
              <div class="detail-label">游戏名称</div>
              <div class="detail-value">{{ selectedBackup.nvramName }}</div>
            </div>
            <div class="detail-item">
              <div class="detail-label">PS版本</div>
              <div class="detail-value">{{ selectedBackup.psVersion.version }} (Build: {{ selectedBackup.psVersion.build }})</div>
            </div>
            <div class="detail-item">
              <div class="detail-label">JChip版本</div>
              <div class="detail-value">{{ selectedBackup.jchipVersion.version }} (Build: {{ selectedBackup.jchipVersion.build }})</div>
            </div>
            <div class="detail-item">
              <div class="detail-label">备份时间</div>
              <div class="detail-value">{{ new Date(selectedBackup.backupTime).toLocaleString() }}</div>
            </div>
            <div class="detail-item">
              <div class="detail-label">文件数量</div>
              <div class="detail-value">{{ selectedBackup.fileList.length }}</div>
            </div>
            <div class="detail-item">
              <div class="detail-label">文件总大小</div>
              <div class="detail-value">{{ selectedBackup.totalSize }}</div>
            </div>

            <!-- 还原选项 -->
            <div class="restore-options">
              <h4 class="options-title">还原选项</h4>
              
              <div class="restore-option-item">
                <div class="option-content">
                  <div class="option-icon clear-icon">🗑️</div>
                  <div class="option-label">还原时清除NVRAM</div>
                </div>
                <label class="toggle-switch">
                  <input type="checkbox" v-model="clearNvramOnRestore">
                  <span class="toggle-slider"></span>
                </label>
              </div>
              
              <div class="restore-option-item">
                <div class="option-content">
                  <div class="option-icon clear-icon">🗑️</div>
                  <div class="option-label">还原时清除当前备份</div>
                </div>
                <label class="toggle-switch">
                  <input type="checkbox" v-model="clearBackupOnRestore">
                  <span class="toggle-slider"></span>
                </label>
              </div>
            </div>
            
            <!-- 操作按钮区域 -->
            <div class="restore-button-container">
              <button class="restore-button" @click="handleRestore">
                <span class="restore-icon">🔄</span>
                <span class="restore-text">还原此备份</span>
              </button>
              <button class="delete-button" @click="handleShowConfirm">
                <span class="delete-icon">🗑️</span>
                <span class="delete-text">删除此备份</span>
              </button>
            </div>
          </div>
        </div>
      </div>
      <!-- 未选中备份时的提示 -->
      <div v-else class="backup-detail-placeholder">
        <div class="info-card">
          <h3 class="list-title">备份详情</h3>
          <div class="placeholder-content">
            <p>请从左侧选择一个备份查看详情</p>
          </div>
        </div>
      </div>
    </div>
  </div>
  
  <!-- 确认删除对话框 -->
  <div v-if="showConfirmDialog" class="confirm-dialog-overlay">
    <div class="confirm-dialog">
      <h3 class="confirm-dialog-title">确认删除备份</h3>
      <p class="confirm-dialog-message">
        确定要删除备份 ID: {{ selectedBackup?.id }} 吗？此操作不可恢复。
      </p>
      <div class="confirm-dialog-buttons">
        <button class="confirm-dialog-cancel" @click="handleCancelDelete">
          取消
        </button>
        <button class="confirm-dialog-delete" @click="handleConfirmDelete">
          确定删除
        </button>
      </div>
    </div>
  </div>
</template>
