<template>
  <div class="settings-view">
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
  padding: 1rem;
}

.settings-card {
  width: 100%;
  background: #ffffff;
  border: 0.0625rem solid #e2e7e6;
  border-radius: 0.875rem;
  overflow: hidden;
  margin-bottom: 1rem;
}
</style>

