<!-- 这一整个页面 LLM 的具体字段都是根据对象的结构自动生成的 -->
<template>
  <div class="llm-config-setting">
    <div class="llm-config-setting__header">
      <div class="llm-config-setting__title">LLM 配置</div>
      <div class="llm-config-setting__desc">根据后端返回的配置结构动态展示当前 provider 的字段</div>
    </div>

    <div v-if="providerNames.length" class="llm-config-setting__body">
      <div class="field">
        <span class="field__label">当前提供商</span>
        <button type="button" class="provider-selector" @click="isProviderPickerOpen = true">
          <span class="provider-logo">{{ currentProviderMeta.mark }}</span>
          <span class="provider-selector__content">
            <strong>{{ currentProviderMeta.name }}</strong>
            <small>{{ currentProviderMeta.description }}</small>
          </span>
          <span class="provider-selector__action">更换</span>
        </button>
      </div>

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

    <Teleport to="body">
      <Transition name="provider-picker">
        <div v-if="isProviderPickerOpen" class="provider-picker-mask" @click.self="closeProviderPicker">
          <section class="provider-picker-card" role="dialog" aria-modal="true" aria-label="连接提供商">
            <header class="provider-picker-header">
              <h3>连接提供商</h3>
              <button type="button" aria-label="关闭" @click="closeProviderPicker">×</button>
            </header>

            <label class="provider-search">
              <svg class="provider-search__icon" viewBox="0 0 24 24" aria-hidden="true">
                <circle cx="11" cy="11" r="6.5" />
                <path d="m16 16 4 4" />
              </svg>
              <input v-model="providerSearch" type="search" aria-label="搜索提供商" placeholder="搜索提供商" autofocus />
            </label>

            <div class="provider-picker-label">可用提供商</div>
            <div class="provider-list">
              <button
                v-for="provider in filteredProviders"
                :key="provider.id"
                type="button"
                class="provider-item"
                :class="{ 'provider-item--active': provider.id === currentProvider }"
                @click="selectProvider(provider.id)"
              >
                <span class="provider-logo">{{ provider.mark }}</span>
                <span class="provider-item__content">
                  <strong>{{ provider.name }}</strong>
                  <small>{{ provider.description }}</small>
                </span>
                <span v-if="provider.id === currentProvider" class="provider-item__check">✓</span>
              </button>

              <div v-if="!filteredProviders.length" class="provider-list__empty">没有匹配的提供商</div>
            </div>
          </section>
        </div>
      </Transition>
    </Teleport>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { pushNotice } from '../../../composables/useNoticeCenter'

type LlmConfigObject = Record<string, unknown>
type ProviderObject = Record<string, string>
type ProviderMeta = { id: string; name: string; description: string; mark: string }

const providerCatalog: Record<string, Omit<ProviderMeta, 'id'>> = {
  deepseek: { name: 'DeepSeek', description: '高性价比推理与通用模型', mark: 'D' },
  openai: { name: 'OpenAI', description: 'GPT 系列模型', mark: '◎' },
  anthropic: { name: 'Anthropic', description: 'Claude 系列模型', mark: 'AI' },
  google: { name: 'Google Gemini', description: 'Gemini 多模态模型', mark: '✦' },
  xai: { name: 'xAI', description: 'Grok 系列模型', mark: 'X' },
  openrouter: { name: 'OpenRouter', description: '一个接口连接多家模型', mark: '↔' },
  mistral: { name: 'Mistral AI', description: '高效的欧洲大模型平台', mark: 'M' },
  minimax: { name: 'MiniMax', description: '面向 Agent 的高性价比模型', mark: 'MM' },
  perplexity: { name: 'Perplexity', description: '带实时搜索能力的 Sonar 模型', mark: 'P' },
  groq: { name: 'Groq', description: 'GroqCloud 高速模型推理', mark: 'G' },
  cerebras: { name: 'Cerebras', description: '超高速模型推理服务', mark: 'C' },
  nvidia: { name: 'NVIDIA NIM', description: 'NVIDIA 托管模型服务', mark: 'N' },
  together: { name: 'Together AI', description: '丰富的开源模型服务', mark: 'T' },
  fireworks: { name: 'Fireworks AI', description: '快速的生成式 AI 推理', mark: 'F' },
  moonshot: { name: 'Moonshot AI', description: 'Kimi 系列模型', mark: 'M' },
  zhipu: { name: '智谱 AI', description: 'GLM 系列模型', mark: '智' },
  dashscope: { name: '阿里云百炼', description: '通义千问系列模型', mark: 'Q' },
  siliconflow: { name: '硅基流动', description: '多模型推理平台', mark: 'S' },
  ollama: { name: 'Ollama', description: '在本机运行开源模型', mark: 'O' },
  custom: { name: '自定义模型', description: '连接 OpenAI-compatible 接口', mark: '✣' },
}

const configState = ref<LlmConfigObject>({})
const currentProvider = ref('')
const isLoaded = ref(false)
const isProviderPickerOpen = ref(false)
const providerSearch = ref('')

const providerNames = computed(() => {
  const names = Object.keys(configState.value).filter((key) => key !== 'current_provider')
  return [
    ...Object.keys(providerCatalog).filter((key) => names.includes(key)),
    ...names.filter((key) => !(key in providerCatalog)),
  ]
})

const providers = computed<ProviderMeta[]>(() => providerNames.value.map((id) => ({
  id,
  ...(providerCatalog[id] ?? { name: id, description: 'OpenAI-compatible provider', mark: id[0]?.toUpperCase() ?? '?' }),
})))

const currentProviderMeta = computed<ProviderMeta>(() => providers.value.find(
  (provider) => provider.id === currentProvider.value,
) ?? { id: '', name: '选择提供商', description: '配置模型连接', mark: '?' })

const filteredProviders = computed(() => {
  const keyword = providerSearch.value.trim().toLocaleLowerCase()
  if (!keyword) return providers.value
  return providers.value.filter((provider) => `${provider.name} ${provider.id} ${provider.description}`
    .toLocaleLowerCase()
    .includes(keyword))
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

function closeProviderPicker() {
  isProviderPickerOpen.value = false
  providerSearch.value = ''
}

async function selectProvider(provider: string) {
  currentProvider.value = provider
  configState.value.current_provider = provider
  closeProviderPicker()
  if (isLoaded.value) await saveConfig()
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

.provider-selector {
  width: 100%;
  display: flex;
  align-items: center;
  gap: 0.75rem;
  min-height: 3.75rem;
  padding: 0.625rem 0.75rem;
  border: 0.0625rem solid #d7dfdd;
  border-radius: 0.75rem;
  background: #fff;
  color: #2f3a39;
  text-align: left;
  cursor: pointer;
}

.provider-selector:hover {
  border-color: #9fbcb4;
  background: #f9fcfb;
}

.provider-logo {
  width: 2rem;
  height: 2rem;
  flex: 0 0 2rem;
  display: grid;
  place-items: center;
  border-radius: 0.6rem;
  background: #eef4f2;
  color: #344944;
  font-size: 0.875rem;
  font-weight: 700;
}

.provider-selector__content,
.provider-item__content {
  min-width: 0;
  display: grid;
  gap: 0.15rem;
}

.provider-selector__content strong,
.provider-item__content strong {
  font-size: 0.8125rem;
  font-weight: 600;
}

.provider-selector__content small,
.provider-item__content small {
  color: #7a8584;
  font-size: 0.7rem;
}

.provider-selector__action {
  margin-left: auto;
  color: #4f756b;
  font-size: 0.75rem;
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

.provider-picker-mask {
  position: fixed;
  inset: 0;
  z-index: 1200;
  display: grid;
  place-items: center;
  padding: 1.5rem;
  background: rgba(25, 34, 32, 0.28);
  backdrop-filter: blur(0.2rem);
}

.provider-picker-card {
  width: min(34rem, 100%);
  max-height: min(38rem, calc(100vh - 3rem));
  display: flex;
  flex-direction: column;
  overflow: hidden;
  border: 0.0625rem solid #e1e8e6;
  border-radius: 1rem;
  background: #fff;
  box-shadow: 0 1.5rem 4rem rgba(32, 48, 43, 0.2);
}

.provider-picker-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 1rem 1.125rem 0.75rem;
}

.provider-picker-header h3 {
  margin: 0;
  color: #263330;
  font-size: 1.05rem;
}

.provider-picker-header button {
  width: 2rem;
  height: 2rem;
  border: 0;
  border-radius: 0.5rem;
  background: transparent;
  color: #71807c;
  font-size: 1.5rem;
  line-height: 1;
  cursor: pointer;
}

.provider-picker-header button:hover {
  background: #f1f5f4;
}

.provider-search {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  margin: 0 0.75rem;
  padding: 0 0.75rem;
  border: 0.0625rem solid #9db8ff;
  border-radius: 0.7rem;
  color: #75817e;
}

.provider-search__icon {
  width: 1.125rem;
  height: 1.125rem;
  flex: 0 0 1.125rem;
  display: block;
  fill: none;
  stroke: currentColor;
  stroke-width: 1.8;
  stroke-linecap: round;
}

.provider-search input {
  width: 100%;
  min-height: 2.6rem;
  border: 0;
  outline: 0;
  background: transparent;
  color: #293633;
  font: inherit;
  font-size: 0.8125rem;
}

.provider-picker-label {
  padding: 1rem 1.125rem 0.45rem;
  color: #7a8584;
  font-size: 0.75rem;
}

.provider-list {
  overflow: auto;
  padding: 0 0.5rem 0.75rem;
}

.provider-item {
  width: 100%;
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.65rem 0.75rem;
  border: 0;
  border-radius: 0.65rem;
  background: transparent;
  color: #293633;
  text-align: left;
  cursor: pointer;
}

.provider-item:hover,
.provider-item--active {
  background: #f1f5f4;
}

.provider-item__check {
  margin-left: auto;
  color: #3e7969;
  font-weight: 700;
}

.provider-list__empty {
  padding: 2rem;
  color: #89938f;
  text-align: center;
  font-size: 0.8rem;
}

.provider-picker-enter-active,
.provider-picker-leave-active {
  transition: opacity 0.16s ease;
}

.provider-picker-enter-from,
.provider-picker-leave-to {
  opacity: 0;
}
</style>
