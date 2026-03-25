<!-- 这一整个页面 LLM 的具体字段都是根据对象的结构自动生成的 -->
<template>
  <div class="llm-config-setting">
    <div class="llm-config-setting__header">
      <div class="llm-config-setting__title">LLM 配置</div>
      <div class="llm-config-setting__desc">根据后端返回的配置结构动态展示当前 provider 的字段</div>
    </div>

    <div v-if="providerNames.length" class="llm-config-setting__body">
      <label class="field">
        <span class="field__label">当前提供商</span>
        <select v-model="currentProvider" class="field__control" @change="handleProviderChange">
          <option v-for="provider in providerNames" :key="provider" :value="provider">
            {{ provider }}
          </option>
        </select>
      </label>

      <div class="provider-panel">
        <div class="provider-panel__title">{{ currentProvider }}</div>

        <div v-for="field in currentProviderFields" :key="field.key" class="provider-field">
          <div class="provider-field__key">{{ field.key }}</div>
          <input
            v-model="field.valueRef.value"
            class="provider-field__input"
            type="text"
            @blur="saveConfig()"
          />
        </div>
      </div>
    </div>

    <div v-else class="llm-config-setting__empty">暂无可用的 provider 配置</div>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { pushNotice } from '../../../composables/useNoticeCenter'

type LlmConfigObject = Record<string, unknown>
type ProviderObject = Record<string, string>

const configState = ref<LlmConfigObject>({})
const currentProvider = ref('')
const isLoaded = ref(false)

const providerNames = computed(() => {
  return Object.keys(configState.value).filter((key) => key !== 'current_provider')
})

const currentProviderConfig = computed<ProviderObject>(() => {
  const provider = currentProvider.value
  const raw = configState.value[provider]

  if (!raw || typeof raw !== 'object' || Array.isArray(raw)) {
    return {}
  }

  return raw as ProviderObject
})

const currentProviderFields = computed(() => {
  return Object.keys(currentProviderConfig.value).map((key) => ({
    key,
    valueRef: computed({
      get: () => currentProviderConfig.value[key] ?? '',
      set: (value: string) => {
        const provider = currentProvider.value
        const raw = configState.value[provider]
        if (!raw || typeof raw !== 'object' || Array.isArray(raw)) {
          configState.value[provider] = { [key]: value }
          return
        }

        ;(raw as ProviderObject)[key] = value
      },
    }),
  }))
})

watch(providerNames, (names) => {
  if (!names.length) {
    currentProvider.value = ''
    return
  }

  if (!names.includes(currentProvider.value)) {
    const backendCurrent = configState.value.current_provider
    currentProvider.value = typeof backendCurrent === 'string' && names.includes(backendCurrent)
      ? backendCurrent
      : names[0]
  }
}, { immediate: true })

watch(currentProvider, (provider) => {
  if (provider) {
    configState.value.current_provider = provider
  }
})

async function loadConfig() {
  try {
    const value = await invoke<LlmConfigObject>('get_config', { section: 'llm' })
    configState.value = value
    isLoaded.value = true
  } catch (error) {
    console.error('读取 LLM 配置失败:', error)
    pushNotice('error', '读取 LLM 配置失败')
  }
}

async function saveConfig() {
  try {
    await invoke('save_config', {
      section: 'llm',
      value: configState.value,
    })
    pushNotice('success', 'LLM 配置已保存')
  } catch (error) {
    console.error('保存 LLM 配置失败:', error)
    pushNotice('error', '保存 LLM 配置失败')
  }
}

function handleProviderChange() {
  if (!isLoaded.value) {
    return
  }

  saveConfig()
}

onMounted(loadConfig)
</script>

<style scoped>
.llm-config-setting {
  padding: 1rem;
}

.llm-config-setting__header {
  margin-bottom: 1rem;
}

.llm-config-setting__title {
  font-size: 0.875rem;
  color: #2f3a39;
  font-weight: 600;
}

.llm-config-setting__desc {
  margin-top: 0.25rem;
  color: #7a8584;
  font-size: 0.75rem;
}

.llm-config-setting__body {
  display: grid;
  gap: 1rem;
}

.field {
  display: grid;
  gap: 0.375rem;
}

.field__label {
  font-size: 0.75rem;
  color: #4d5857;
}

.field__control,
.provider-field__input {
  width: 100%;
  min-height: 2.25rem;
  border: 0.0625rem solid #d7dfdd;
  border-radius: 0.625rem;
  background: #fff;
  color: #2f3a39;
  padding: 0.5rem 0.75rem;
  font-size: 0.75rem;
  outline: none;
}

.provider-panel {
  border: 0.0625rem solid #e8edec;
  border-radius: 0.75rem;
  padding: 0.875rem;
  display: grid;
  gap: 0.75rem;
}

.provider-panel__title {
  font-size: 0.8125rem;
  font-weight: 600;
  color: #2f3a39;
}

.provider-field {
  display: grid;
  grid-template-columns: 8rem 1fr;
  gap: 0.75rem;
  align-items: center;
}

.provider-field__key {
  color: #5c6766;
  font-size: 0.75rem;
  word-break: break-word;
}

.llm-config-setting__empty {
  padding: 1rem;
  color: #7a8584;
  font-size: 0.75rem;
}
</style>
