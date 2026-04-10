<template>
  <div class="ghost-disk-list">
    <div v-if="items.length === 0" class="ghost-disk-empty">当前没有可展示的条目</div>

    <article v-for="item in items" :key="item.path" class="ghost-disk-item">
      <div class="ghost-disk-path" :title="item.path">{{ item.path }}</div>
      <div class="ghost-disk-purpose">{{ item.purpose }}</div>
      <div class="ghost-disk-actions">
        <button type="button" class="ghost-disk-action ghost-disk-action-secondary" @click="handleReveal(item.path)">
          打开文件
        </button>
        <button type="button" class="ghost-disk-action ghost-disk-action-danger" @click="handleTrash(item.path)">
          移到回收站
        </button>
      </div>
    </article>
  </div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'

const props = defineProps<{
  items: { path: string, purpose: string }[]
  recordPath: string
}>()

const emit = defineEmits<{
  refresh: []
}>()

async function handleReveal(path: string) {
  await invoke('reveal_storage_box_path', { path })
}

async function handleTrash(path: string) {
  await invoke('trash_storage_box_path', { path })
  emit('refresh')
}
</script>

<style scoped>
.ghost-disk-list {
  flex: 1;
  min-height: 0;
  overflow: auto;
  display: flex;
  flex-direction: column;
  gap: 0.875rem;
}

.ghost-disk-empty {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 1rem;
  background: rgba(246, 249, 248, 0.92);
  border: 1px dashed rgba(206, 216, 212, 0.95);
  color: #7a8b87;
}

.ghost-disk-item {
  display: grid;
  grid-template-columns: minmax(20rem, 2.1fr) minmax(12rem, 1.4fr) auto;
  gap: 1rem;
  align-items: center;
  padding: 1rem 1.125rem;
  border-radius: 1rem;
  background: rgba(255, 255, 255, 0.92);
  border: 1px solid rgba(224, 231, 228, 0.95);
  box-shadow: 0 0.375rem 1.25rem rgba(55, 75, 69, 0.04);
}

.ghost-disk-path,
.ghost-disk-purpose {
  min-width: 0;
  line-height: 1.55;
}

.ghost-disk-path {
  font-size: 0.92rem;
  font-weight: 600;
  color: #243230;
  word-break: break-all;
}

.ghost-disk-purpose {
  font-size: 0.875rem;
  color: #627571;
}

.ghost-disk-actions {
  display: flex;
  align-items: center;
  gap: 0.625rem;
}

.ghost-disk-action {
  border: 0;
  border-radius: 0.75rem;
  padding: 0.625rem 0.875rem;
  font-size: 0.8125rem;
  font-weight: 600;
  cursor: pointer;
}

.ghost-disk-action-secondary {
  background: rgba(37, 51, 48, 0.08);
  color: #314341;
}

.ghost-disk-action-secondary:hover {
  background: rgba(37, 51, 48, 0.12);
}

.ghost-disk-action-danger {
  background: rgba(227, 77, 77, 0.12);
  color: #c13f3f;
}

.ghost-disk-action-danger:hover {
  background: rgba(227, 77, 77, 0.18);
}
</style>
