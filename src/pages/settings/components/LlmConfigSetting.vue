<template>
  <div class="settings-item settings-item--block">
    <div class="settings-item__header">
      <div class="settings-item__title">LLM 配置</div>
      <div class="settings-item__desc">配置当前使用的模型服务商，以及各服务商对应的参数</div>
    </div>

    <div class="llm-config">
      <label class="field">
        <span class="field__label">当前服务商</span>
        <select v-model="currentProvider" class="field__control">
          <option v-for="name in providerNames" :key="name" :value="name">{{ name }}</option>
        </select>
      </label>

      <div class="provider-tools">
        <input
          v-model.trim="newProviderName"
          class="field__control"
          type="text"
          placeholder="新增服务商名称，例如 openai"
          @keydown.enter.prevent="addProvider"
        />
        <button class="settings-item__action" @click="addProvider">新增服务商</button>
      </div>

      <label class="field">
        <span class="field__label">{{ currentProvider }} 配置（JSON）</span>
        <textarea
          v-model="providerJsonText"
          class="field__control field__textarea"
          spellcheck="false"
          placeholder='例如：{"api_key":"", "base_url":"", "model":""}'
        />
      </label>

      <div class="actions">
        <button class="settings-item__action" @click="save">保存配置</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { pushNotice } from '../../../composables/useNoticeCenter'

type ProviderConfig = Record<string, string>

const SECTION = 'llm'
const KEY = 'config'

const currentProvider = ref('deepseek')
const providerMap = ref<Record<string, ProviderConfig>>({
  deepseek: { api_key: '', base_url: 'https://api.deepseek.com', model: 'deepseek-chat' },
})

const providerJsonText = ref('{}')
const newProviderName = ref('')

const providerNames = computed(() => {
  const names = Object.keys(providerMap.value)
  return names.length ? names : ['deepseek']
})

function toBackendShape() {
  return {
    current_provider: currentProvider.value,
    ...providerMap.value,
  }
}

function normalizeFromBackend(input: unknown): { currentProvider: string; providers: Record<string, ProviderConfig> } {
  if (!input || typeof input !== 'object' || Array.isArray(input)) {
    return {
      currentProvider: 'deepseek',
      providers: { deepseek: { api_key: '', base_url: 'https://api.deepseek.com', model: 'deepseek-chat' } },
    }
  }

  const raw = input as Record<string, unknown>
  const providers: Record<string, ProviderConfig> = {}

  for (const [key, value] of Object.entries(raw)) {
    if (key === 'current_provider') continue
    if (!value || typeof value !== 'object' || Array.isArray(value)) continue

    const obj = value as Record<string, unknown>
    const conf: ProviderConfig = {}
    for (const [k, v] of Object.entries(obj)) {
      conf[k] = typeof v === 'string' ? v : JSON.stringify(v)
    }
    providers[key] = conf
  }

  const firstProvider = Object.keys(providers)[0] ?? 'deepseek'
  const parsedCurrent = typeof raw.current_provider === 'string' && raw.current_provider.trim()
    ? raw.current_provider.trim()
    : firstProvider

  if (!providers[parsedCurrent]) {
    providers[parsedCurrent] = {}
  }

  if (Object.keys(providers).length === 0) {
    providers.deepseek = { api_key: '', base_url: 'https://api.deepseek.com', model: 'deepseek-chat' }
  }

  return {
    currentProvider: parsedCurrent,
    providers,
  }
}

function syncEditorFromCurrentProvider() {
  const current = providerMap.value[currentProvider.value] ?? {}
  providerJsonText.value = JSON.stringify(current, null, 2)
}

watch(currentProvider, () => {
  if (!providerMap.value[currentProvider.value]) {
    providerMap.value[currentProvider.value] = {}
  }
  syncEditorFromCurrentProvider()
})

function addProvider() {
  const name = newProviderName.value.trim()
  if (!name) {
    pushNotice('error', '服务商名称不能为空')
    return
  }

  if (!providerMap.value[name]) {
    providerMap.value[name] = {}
  }

  currentProvider.value = name
  newProviderName.value = ''
  pushNotice('success', `已新增服务商：${name}`)
}

async function load() {
  try {
    const value = await invoke<unknown>('get_config', {
      section: SECTION,
      key: KEY,
    })

    const normalized = normalizeFromBackend(value)
    providerMap.value = normalized.providers
    currentProvider.value = normalized.currentProvider
    syncEditorFromCurrentProvider()
  } catch (error) {
    console.error('读取 LLM 配置失败:', error)
    syncEditorFromCurrentProvider()
    pushNotice('error', '读取 LLM 配置失败')
  }
}

async function save() {
  let parsedCurrentConfig: ProviderConfig
  try {
    const parsed = JSON.parse(providerJsonText.value) as unknown
    if (!parsed || typeof parsed !== 'object' || Array.isArray(parsed)) {
      pushNotice('error', '当前服务商配置必须是 JSON 对象')
      return
    }

    parsedCurrentConfig = {}
    for (const [k, v] of Object.entries(parsed as Record<string, unknown>)) {
      parsedCurrentConfig[k] = typeof v === 'string' ? v : JSON.stringify(v)
    }
  } catch {
    pushNotice('error', '当前服务商配置不是合法 JSON')
    return
  }

  providerMap.value[currentProvider.value] = parsedCurrentConfig

  try {
    await invoke('save_config', {
      section: SECTION,
      key: KEY,
      value: toBackendShape(),
    })
    pushNotice('success', 'LLM 配置已保存')
  } catch (error) {
    console.error('保存 LLM 配置失败:', error)
    pushNotice('error', '保存 LLM 配置失败')
  }
}

onMounted(load)
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

.settings-item--block {
  display: block;
  border-top: 0.0625rem solid #eef3f2;
}

.settings-item__header {
  margin-bottom: 0.75rem;
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

.llm-config {
  display: grid;
  gap: 0.75rem;
}

.field {
  display: grid;
  gap: 0.375rem;
}

.field__label {
  color: #4d5857;
  font-size: 0.75rem;
}

.field__control {
  width: 100%;
  min-height: 2rem;
  border-radius: 0.5rem;
  border: 0.0625rem solid #d7dfdd;
  background: #fff;
  color: #2f3a39;
  font-size: 0.75rem;
  padding: 0.375rem 0.625rem;
  outline: none;
}

.field__control:focus {
  border-color: #b9c8c4;
}

.field__textarea {
  min-height: 8.5rem;
  resize: vertical;
  font-family: Consolas, 'Courier New', monospace;
}

.provider-tools {
  display: flex;
  gap: 0.5rem;
}

.actions {
  display: flex;
  justify-content: flex-end;
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
  white-space: nowrap;
}

.settings-item__action:hover:not(:disabled) {
  background: #eef3f2;
  border-color: #ccd6d4;
}
</style>
