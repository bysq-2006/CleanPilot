<template>
  <div>
    <div class="settings-item">
      <div class="settings-item__content">
        <div class="settings-item__title">存储目录</div>
        <div class="settings-item__desc">配置、历史记录与任务文件</div>
      </div>

      <button class="settings-item__action" @click="openDirectory('open_storage_directory', '存储目录')">打开目录</button>
    </div>

    <div class="settings-item">
      <div class="settings-item__content">
        <div class="settings-item__title">日志目录</div>
        <div class="settings-item__desc">查看、导出或清理应用运行日志</div>
      </div>

      <button class="settings-item__action" @click="openDirectory('open_log_directory', '日志目录')">打开目录</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { pushNotice } from '../../../composables/useNoticeCenter'

async function openDirectory(command: string, name: string) {
  try {
    await invoke(command)
    pushNotice('success', `已打开${name}`)
  } catch (error) {
    console.error(`打开${name}失败:`, error)
    pushNotice('error', `打开${name}失败`)
  }
}
</script>

<style scoped>
.settings-item {
  min-height: 4rem;
  padding: 0.875rem 1rem;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 1rem;
}

.settings-item + .settings-item {
  border-top: 0.0625rem solid #e2e7e6;
}

.settings-item__content {
  min-width: 0;
}

.settings-item__title {
  font-size: 0.875rem;
  color: #2f3a39;
  font-weight: 500;
}

.settings-item__desc {
  margin-top: 0.25rem;
  color: #7a8584;
  font-size: 0.75rem;
}

.settings-item__action {
  height: 2rem;
  padding: 0 0.875rem;
  border-radius: 0.625rem;
  border: 0.0625rem solid #d7dfdd;
  background: #f8faf9;
  color: #4d5857;
  font-size: 0.75rem;
  cursor: pointer;
  transition: all 0.15s ease;
}

.settings-item__action:hover:not(:disabled) {
  background: #eef3f2;
  border-color: #ccd6d4;
}
</style>
