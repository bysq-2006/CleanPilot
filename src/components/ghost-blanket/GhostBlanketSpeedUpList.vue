<!-- 展示并管理电脑变快任务中的开机项开关与进程结束。 -->
<template>
  <div class="speed-up-list">
    <div v-if="loading" class="state">正在读取当前状态…</div>
    <div v-else-if="error" class="state state-error">{{ error }}</div>
    <div v-else-if="!startupItems.length && !processes.length" class="state">当前没有可展示的条目</div>

    <template v-else>
      <section v-if="startupItems.length" class="section">
        <div class="section-head">
          <span class="section-title">开机自启动</span>
          <span class="section-meta">打开任务时会重新读取当前开关状态</span>
        </div>

        <div
          v-for="item in startupItems"
          :key="`${item.location}:${item.name}`"
          class="row"
          :class="{ muted: !item.found }"
          :title="item.command"
        >
          <div class="row-main">
            <span class="name">{{ item.name }}</span>
            <span class="reason">{{ item.reason }}</span>
          </div>
          <div class="row-side">
            <span class="hint">{{ item.found ? formatLocation(item.location) : '已经不存在' }}</span>
            <button
              type="button"
              class="switch"
              :class="{ on: item.found && item.enabled }"
              :disabled="!item.found || pendingKey === startupKey(item)"
              :aria-checked="item.found && item.enabled"
              role="switch"
              :aria-label="`切换 ${item.name} 开机自启`"
              @click="toggleStartup(item)"
            >
              <span class="switch-knob" />
            </button>
          </div>
        </div>
      </section>

      <section v-if="processes.length" class="section">
        <div class="section-head">
          <span class="section-title">现在先关掉</span>
          <span class="section-meta">按程序路径匹配当前仍在运行的进程</span>
        </div>

        <div
          v-for="item in processes"
          :key="`${item.path}:${item.name}`"
          class="row"
          :class="{ muted: !item.running }"
          :title="item.path"
        >
          <div class="row-main">
            <span class="name">{{ item.name }}</span>
            <span class="reason">{{ item.reason }}</span>
          </div>
          <div class="row-side">
            <span class="hint">{{ processHint(item) }}</span>
            <button
              type="button"
              class="end-button"
              :disabled="!item.running || pendingKey === processKey(item)"
              @click="requestEndProcess(item)"
            >
              结束
            </button>
          </div>
        </div>
      </section>
    </template>

    <ConfirmDialog
      :open="confirmItem !== null"
      title="结束进程"
      :message="confirmMessage"
      :detail="confirmItem?.path"
      @confirm="confirmEndProcess"
      @cancel="confirmItem = null"
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

interface LiveStartupItem {
  name: string
  location: string
  command: string
  reason: string
  enabled: boolean
  found: boolean
}

interface LiveProcessItem {
  name: string
  path: string
  reason: string
  running: boolean
  instance_count: number
  memory: number
}

const loading = ref(true)
const error = ref('')
const pendingKey = ref('')
const startupItems = ref<LiveStartupItem[]>([])
const processes = ref<LiveProcessItem[]>([])
const confirmItem = ref<LiveProcessItem | null>(null)

const confirmMessage = computed(() => {
  const item = confirmItem.value
  if (!item) return ''
  const count = Math.max(1, item.instance_count)
  return `确认结束当前正在运行的 ${count} 个 ${item.name} 吗？`
})

const startupKey = (item: LiveStartupItem) => `startup:${item.location}:${item.name}`
const processKey = (item: LiveProcessItem) => `process:${item.path}:${item.name}`

const formatLocation = (location: string) => {
  const labels: Record<string, string> = {
    hkcu_run: 'HKCU · Run',
    hkcu_run_once: 'HKCU · RunOnce',
    hklm_run: 'HKLM · Run',
    hklm_run_once: 'HKLM · RunOnce',
    hklm_wow_run: 'HKLM32 · Run',
    hklm_wow_run_once: 'HKLM32 · RunOnce',
    user_folder: '用户启动文件夹',
    common_folder: '公共启动文件夹',
  }
  return labels[location] ?? location
}

const formatBytes = (bytes: number) => {
  if (!Number.isFinite(bytes) || bytes <= 0) return ''
  if (bytes < 1024) return `${bytes} B`
  const units = ['KB', 'MB', 'GB', 'TB']
  let value = bytes / 1024
  let unitIndex = 0
  while (value >= 1024 && unitIndex < units.length - 1) {
    value /= 1024
    unitIndex += 1
  }
  return `${value.toFixed(value >= 10 ? 0 : 1)} ${units[unitIndex]}`
}

const processHint = (item: LiveProcessItem) => {
  if (!item.running) return '当前没在运行'
  const memory = formatBytes(item.memory)
  if (item.instance_count > 1) {
    return memory ? `${item.instance_count} 个在运行 · ${memory}` : `${item.instance_count} 个在运行`
  }
  return memory ? `正在运行 · ${memory}` : '正在运行'
}

async function loadItems() {
  loading.value = true
  error.value = ''

  try {
    const result = await invoke<{
      processes: LiveProcessItem[]
      startup_items: LiveStartupItem[]
    }>('get_speed_up_items', { path: props.recordPath })
    startupItems.value = result.startup_items
    processes.value = result.processes
  }
  catch (err) {
    error.value = err instanceof Error ? err.message : String(err)
  }
  finally {
    loading.value = false
  }
}

async function toggleStartup(item: LiveStartupItem) {
  if (!item.found) return

  const key = startupKey(item)
  pendingKey.value = key
  try {
    const next = await invoke<LiveStartupItem>('set_speed_up_startup', {
      recordPath: props.recordPath,
      name: item.name,
      location: item.location,
      enabled: !item.enabled,
    })
    startupItems.value = startupItems.value.map(current =>
      startupKey(current) === key ? next : current,
    )
  }
  catch (err) {
    pushNotice('error', `切换开机项失败：${String(err)}`)
  }
  finally {
    pendingKey.value = ''
  }
}

function requestEndProcess(item: LiveProcessItem) {
  if (!item.running) return
  confirmItem.value = item
}

async function confirmEndProcess() {
  const item = confirmItem.value
  confirmItem.value = null
  if (!item) return

  const key = processKey(item)
  pendingKey.value = key
  try {
    const next = await invoke<LiveProcessItem>('end_speed_up_process', {
      recordPath: props.recordPath,
      name: item.name,
      path: item.path,
    })
    processes.value = processes.value.map(current =>
      processKey(current) === key ? next : current,
    )
    if (!next.running) {
      pushNotice('success', `已结束 ${item.name}`)
    }
  }
  catch (err) {
    const message = err instanceof Error ? err.message : String(err)
    pushNotice('error', `结束进程失败：${message}`)
    await loadItems()
  }
  finally {
    pendingKey.value = ''
  }
}

onMounted(() => {
  void loadItems()
})
</script>

<style scoped>
.speed-up-list {
  flex: 1;
  min-height: 0;
  overflow: auto;
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.state {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 1rem;
  background: rgba(246, 249, 248, 0.92);
  border: 1px dashed rgba(206, 216, 212, 0.95);
  color: #7a8b87;
}

.state-error {
  color: #cb4d4d;
  border-color: rgba(223, 142, 142, 0.45);
}

.section {
  display: flex;
  flex-direction: column;
}

.section-head {
  display: flex;
  align-items: baseline;
  justify-content: space-between;
  gap: 0.75rem;
  padding: 0 0.15rem 0.5rem;
  border-bottom: 0.0625rem solid #e8eef3;
}

.section-title {
  font-size: 0.75rem;
  font-weight: 700;
  letter-spacing: 0.04em;
  text-transform: uppercase;
  color: #64748b;
}

.section-meta {
  font-size: 0.6875rem;
  color: #94a3b8;
}

.row {
  display: grid;
  grid-template-columns: minmax(0, 1fr) auto;
  gap: 1rem;
  align-items: center;
  min-height: 3.1rem;
  padding: 0.55rem 0.15rem;
  border-bottom: 0.0625rem dashed #eef2f6;
}

.row:last-child {
  border-bottom: 0;
}

.row.muted .name {
  color: #94a3b8;
}

.row-main {
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 0.15rem;
}

.name {
  font-size: 0.875rem;
  font-weight: 650;
  color: #1e293b;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.reason {
  font-size: 0.75rem;
  color: #7a8b87;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.row-side {
  display: flex;
  align-items: center;
  gap: 0.75rem;
}

.hint {
  font-size: 0.6875rem;
  color: #94a3b8;
  white-space: nowrap;
}

.switch {
  width: 2.35rem;
  height: 1.3rem;
  padding: 0.12rem;
  border: 0;
  border-radius: 999px;
  background: #dbe3ea;
  cursor: pointer;
  transition: background-color 0.16s ease;
}

.switch.on {
  background: #d97706;
}

.switch:disabled {
  cursor: not-allowed;
  opacity: 0.45;
}

.switch-knob {
  display: block;
  width: 1.05rem;
  height: 1.05rem;
  border-radius: 999px;
  background: #ffffff;
  transform: translateX(0);
  transition: transform 0.16s ease;
}

.switch.on .switch-knob {
  transform: translateX(1.05rem);
}

.end-button {
  height: 1.75rem;
  padding: 0 0.75rem;
  border: 0.0625rem solid #e2e8f0;
  border-radius: 999px;
  background: #ffffff;
  color: #334155;
  font-size: 0.75rem;
  cursor: pointer;
}

.end-button:hover:not(:disabled) {
  border-color: #fecaca;
  background: #fff1f2;
  color: #b91c1c;
}

.end-button:disabled {
  cursor: not-allowed;
  opacity: 0.45;
}
</style>
