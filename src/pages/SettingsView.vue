<template>
  <div class="settings-view">
    <section class="settings-card">

      <div class="settings-item">
        <div class="settings-item__content">
          <div class="settings-item__title">存储目录</div>
          <div class="settings-item__desc">打开应用数据存储目录（配置与状态文件）</div>
        </div>

        <button class="settings-item__action" @click="openStorageDir">打开目录</button>
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { pushNotice } from '../composables/useNoticeCenter'

async function openStorageDir() {
  try {
    await invoke('open_storage_directory')
    pushNotice('success', '已打开存储目录')
  } catch (error) {
    console.error('打开存储目录失败:', error)
    pushNotice('error', '打开存储目录失败')
  }
}
</script>

<style scoped>
.settings-view {
  width: 100%;
  height: 100%;
  padding: 1rem;
}

.settings-card {
  width: 100%;
  background: #ffffff;
  border: 0.0625rem solid #e2e7e6;
  border-radius: 0.875rem;
  overflow: hidden;
}

.settings-item {
  min-height: 4rem;
  padding: 0.875rem 1rem;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 1rem;
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

