<script setup lang="ts">
// 接收props
defineProps<{
  backupPath: string;
  statusInfo: {
    level: string;
    message: string;
  };
}>();
</script>

<template>
  <!-- 状态栏 -->
  <footer class="status-bar">
    <div class="status-left">{{ backupPath }}</div>
    <div class="status-right">
      <span class="status-message" :class="statusInfo.level">
        {{ statusInfo.message }}
      </span>
    </div>
  </footer>
</template>
