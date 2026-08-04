<!-- 展示并管理磁盘清理任务中的候选条目。 -->
<template>
  <div class="ghost-disk-list">
    <div v-if="selectedPaths.length > 0" class="ghost-disk-toolbar">
      <span>已选 {{ selectedPaths.length }} 项</span>
      <div class="ghost-disk-toolbar-actions">
        <button type="button" class="ghost-disk-action ghost-disk-action-secondary" @click="toggleSelectAll">
          {{ allSelected ? '取消全选' : '全选' }}
        </button>
        <button type="button" class="ghost-disk-action ghost-disk-action-secondary" @click="invertSelection">
          反选
        </button>
        <button type="button" class="ghost-disk-action ghost-disk-action-danger" @click="requestDeleteSelected">
          删除已选
        </button>
      </div>
    </div>

    <div v-if="items.length === 0" class="ghost-disk-empty">当前没有可展示的条目</div>

    <article v-for="item in items" :key="item.path" class="ghost-disk-item">
      <input
        class="ghost-disk-checkbox"
        type="checkbox"
        :checked="selectedPaths.includes(item.path)"
        @click.stop
        @change="toggleSelection(item.path)"
      />
      <div class="ghost-disk-item-content">
        <div class="ghost-disk-path" :title="item.path">{{ item.path }}</div>
        <div class="ghost-disk-purpose">{{ item.purpose }}</div>
        <div class="ghost-disk-actions">
          <button type="button" class="ghost-disk-action ghost-disk-action-secondary" @click="handleReveal(item.path)">
            打开文件夹
          </button>
          <button type="button" class="ghost-disk-action ghost-disk-action-danger" @click="requestRecycle(item.path)">
            移入回收站
          </button>
        </div>
      </div>
    </article>

    <ConfirmDialog
      :open="confirmRequest !== null"
      :title="confirmContent.title"
      :message="confirmContent.message"
      :detail="confirmContent.detail"
      @confirm="handleConfirm"
      @cancel="confirmRequest = null"
    />
  </div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { computed, onMounted, ref } from 'vue'
import { pushNotice } from '../../composables/useNoticeCenter'
import ConfirmDialog from '../ConfirmDialog.vue'

const props = defineProps<{
  recordPath: string
}>()

interface DiskCleanupItem {
  path: string
  purpose: string
}

type ConfirmRequest =
  | { kind: 'recycle', path: string }
  | { kind: 'delete-selected' }

const items = ref<DiskCleanupItem[]>([])
const selectedPaths = ref<string[]>([])
const confirmRequest = ref<ConfirmRequest | null>(null)
const allSelected = computed(() => items.value.length > 0 && selectedPaths.value.length === items.value.length)
const confirmContent = computed(() => confirmRequest.value?.kind === 'recycle'
  ? {
      title: '移入回收站',
      message: '确认将这个项目移入回收站吗？',
      detail: confirmRequest.value.path,
    }
  : {
      title: '删除已选',
      message: `确认从当前清单中删除已选的 ${selectedPaths.value.length} 项吗？`,
      detail: undefined,
    })

onMounted(async () => {
  items.value = await invoke<DiskCleanupItem[]>('get_disk_cleanup_items', { path: props.recordPath })
})

async function handleReveal(path: string) {
  try {
    await invoke('reveal_storage_box_path', { recordPath: props.recordPath, path })
  }
  catch (error) {
    pushNotice('error', `打开位置失败：${String(error)}`)
  }
}

async function recycleItem(path: string) {
  try {
    await invoke('recycle_disk_cleanup_item', { recordPath: props.recordPath, path })
    items.value = items.value.filter(item => item.path !== path)
    selectedPaths.value = selectedPaths.value.filter(item => item !== path)
  }
  catch (error) {
    pushNotice('error', `移入回收站失败：${String(error)}`)
  }
}

function requestRecycle(path: string) {
  confirmRequest.value = { kind: 'recycle', path }
}

function toggleSelection(path: string) {
  selectedPaths.value = selectedPaths.value.includes(path)
    ? selectedPaths.value.filter(item => item !== path)
    : [...selectedPaths.value, path]
}

function toggleSelectAll() {
  selectedPaths.value = allSelected.value ? [] : items.value.map(item => item.path)
}

function invertSelection() {
  const selected = new Set(selectedPaths.value)
  selectedPaths.value = items.value
    .filter(item => !selected.has(item.path))
    .map(item => item.path)
}

function requestDeleteSelected() {
  if (selectedPaths.value.length === 0)
    return

  confirmRequest.value = { kind: 'delete-selected' }
}

async function deleteSelected() {
  const selected = new Set(selectedPaths.value)
  const nextItems = items.value.filter(item => !selected.has(item.path))

  await invoke('save_disk_cleanup_items', {
    path: props.recordPath,
    items: nextItems,
  })

  items.value = nextItems
  selectedPaths.value = []
}

async function handleConfirm() {
  const request = confirmRequest.value
  confirmRequest.value = null

  if (request?.kind === 'recycle')
    await recycleItem(request.path)
  else if (request)
    await deleteSelected()
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

.ghost-disk-toolbar {
  position: sticky;
  top: 0;
  z-index: 1;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 1rem;
  padding: 0.75rem 1rem;
  border: 1px solid rgba(224, 231, 228, 0.95);
  border-radius: 1rem;
  background: rgba(255, 255, 255, 0.96);
  color: #627571;
  font-size: 0.875rem;
}

.ghost-disk-toolbar-actions {
  display: flex;
  align-items: center;
  gap: 0.5rem;
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
  display: flex;
  gap: 1rem;
  align-items: center;
  padding: 1rem 1.125rem;
  border-radius: 1rem;
  background: rgba(255, 255, 255, 0.92);
  border: 1px solid rgba(224, 231, 228, 0.95);
  box-shadow: 0 0.375rem 1.25rem rgba(55, 75, 69, 0.04);
}

.ghost-disk-item-content {
  min-width: 0;
  flex: 1;
  display: grid;
  grid-template-columns: minmax(16rem, 1.25fr) minmax(12rem, 1fr) auto;
  gap: 1rem;
  align-items: center;
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
  white-space: nowrap;
}

.ghost-disk-action {
  border: 0;
  border-radius: 0.75rem;
  padding: 0.625rem 0.875rem;
  font-size: 0.8125rem;
  font-weight: 600;
  cursor: pointer;
}

.ghost-disk-checkbox {
  width: 1rem;
  height: 1rem;
  accent-color: #2e6e5c;
  cursor: pointer;
}

.ghost-disk-action:disabled {
  cursor: not-allowed;
  opacity: 0.5;
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
