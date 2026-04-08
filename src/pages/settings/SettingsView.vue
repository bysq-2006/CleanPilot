<template>
  <div class="settings-view">
    <div class="settings-header-block">
      <div class="settings-header">设置</div>
      <div class="settings-subtitle">统一管理存储目录与模型配置，保持和其他页面一致的浏览体验。</div>
    </div>

    <section class="settings-card">
      <StorageDirectorySetting />
    </section>
    <section class="settings-card">
      <LlmConfigSetting />
    </section>
  </div>
</template>

<script setup lang="ts">
import StorageDirectorySetting from './components/StorageDirectorySetting.vue'
import LlmConfigSetting from './components/LlmConfigSetting.vue'
</script>

<style scoped>
.settings-view {
  width: 100%;
  height: 100%;
  padding: 1.25rem;
  box-sizing: border-box;
  background: #f9fbfc;
  overflow: auto;
}

.settings-header-block {
  padding: 0.25rem 0 1rem;
}

.settings-header {
  font-size: 1.375rem;
  line-height: 1.2;
  font-weight: 700;
  color: #1f2a28;
}

.settings-subtitle {
  margin-top: 0.375rem;
  font-size: 0.875rem;
  line-height: 1.45;
  color: #70827b;
}

.settings-card {
  width: 100%;
  background: #ffffff;
  border: 0.0625rem solid #e2e7e6;
  border-radius: 0.875rem;
  overflow: hidden;
  margin-bottom: 1rem;
  box-shadow: 0 4px 12px rgba(95, 120, 110, 0.05);
}
</style>
