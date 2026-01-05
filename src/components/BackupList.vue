<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

// 导入类型定义
import type { NvramBackup } from '../types/index';

// 接收props
const { backups } = defineProps<{
  backups: NvramBackup[];
}>();

// 定义事件
const emit = defineEmits<{
  updateSelectedBackup: [backup: NvramBackup | null];
}>();

// 选中的备份
const selectedBackup = ref<NvramBackup | null>(null);

// 确认对话框状态
const showConfirmDialog = ref(false);
// NVRAM覆盖确认对话框状态
const showOverwriteConfirm = ref(false);
// 路径不匹配确认对话框状态
const showPathMismatchConfirm = ref(false);

// 选择备份
const selectBackup = (backup: NvramBackup) => {
  selectedBackup.value = backup;
  emit('updateSelectedBackup', backup);
};

// 执行还原操作
const handleRestore = async () => {
  if (!selectedBackup.value) return;
  
  try {
    // 检查NVRAM是否存在
    const nvramExists = await invoke<boolean>("check_nvram_existence");
    
    // 验证游戏路径是否匹配
    const validationResult = await invoke<any>("validate_game_path", {
      backupId: selectedBackup.value.id
    });
    
    if (!validationResult.isMatch) {
      // 如果路径不匹配，显示路径不匹配确认对话框
      showPathMismatchConfirm.value = true;
    } else if (nvramExists) {
      // 如果路径匹配但NVRAM存在，显示覆盖确认对话框
      showOverwriteConfirm.value = true;
    } else {
      // 如果路径匹配且NVRAM不存在，直接执行还原操作
      await executeRestore();
    }
  } catch (error) {
    console.error("还原操作失败:", error);
  }
};

// 执行实际的还原操作
const executeRestore = async () => {
  if (!selectedBackup.value) return;
  
  try {
    const result = await invoke<boolean>("restore_backup", {
      backupId: selectedBackup.value.id
    });
    if (result) {
      console.log("还原成功");
      // 还原成功后，父组件会通过事件监听更新备份列表
    }
  } catch (error) {
    console.error("还原失败:", error);
  }
};

// 处理覆盖确认
const handleOverwriteConfirm = async () => {
  // 关闭确认对话框
  showOverwriteConfirm.value = false;
  // 执行还原操作
  await executeRestore();
};

// 取消覆盖
const handleOverwriteCancel = () => {
  showOverwriteConfirm.value = false;
};

// 处理路径不匹配确认
const handlePathMismatchConfirm = async () => {
  // 关闭确认对话框
  showPathMismatchConfirm.value = false;
  // 检查NVRAM是否存在
  const nvramExists = await invoke<boolean>("check_nvram_existence");
  
  if (nvramExists) {
    // 如果NVRAM存在，显示覆盖确认对话框
    showOverwriteConfirm.value = true;
  } else {
    // 如果NVRAM不存在，直接执行还原操作
    await executeRestore();
  }
};

// 取消路径不匹配
const handlePathMismatchCancel = () => {
  showPathMismatchConfirm.value = false;
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
              <div class="backup-item-size">{{ backup.fileList.length }} files</div>
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
              <div class="detail-label">游戏路径</div>
              <div class="detail-value">{{ selectedBackup.gamePath }}</div>
            </div>
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
  
  <!-- NVRAM覆盖确认对话框 -->
  <div v-if="showOverwriteConfirm" class="confirm-dialog-overlay">
    <div class="confirm-dialog">
      <h3 class="confirm-dialog-title">确认覆盖NVRAM</h3>
      <p class="confirm-dialog-message">
        当前NVRAM已存在，还原操作将覆盖现有数据，是否继续？
      </p>
      <div class="confirm-dialog-buttons">
        <button class="confirm-dialog-cancel" @click="handleOverwriteCancel">
          否
        </button>
        <button class="confirm-dialog-delete" @click="handleOverwriteConfirm">
          是
        </button>
      </div>
    </div>
  </div>
  
  <!-- 路径不匹配确认对话框 -->
  <div v-if="showPathMismatchConfirm" class="confirm-dialog-overlay">
    <div class="confirm-dialog">
      <h3 class="confirm-dialog-title">路径不匹配警告</h3>
      <p class="confirm-dialog-message">
        当前游戏路径与备份路径不匹配，还原操作可能导致数据异常，是否继续？
      </p>
      <div class="confirm-dialog-buttons">
        <button class="confirm-dialog-cancel" @click="handlePathMismatchCancel">
          否
        </button>
        <button class="confirm-dialog-delete" @click="handlePathMismatchConfirm">
          是
        </button>
      </div>
    </div>
  </div>
</template>
