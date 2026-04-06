<template>
  <div class="new-chat-view">
    <section class="entry-shell">
      <div class="brand-block" aria-hidden="true">
        <img src="/robot.svg" alt="" class="brand-logo" />
        <div class="brand-title">CleanPilot</div>
      </div>

      <div class="entry-copy">今天想处理什么问题？</div>

      <div class="entry-composer">
        <div class="entry-input-shell">
          <textarea ref="textareaRef" v-model="inputText" rows="1" class="entry-textarea"
            placeholder="描述你的清理需求，发送后会新建会话并跳转到正式聊天页" :disabled="isSubmitting" @input="resizeTextarea"
            @keydown.enter.exact.prevent="submitEntry" />

          <div class="entry-actions">
            <button type="button" class="scene-button" :disabled="isSubmitting" :aria-expanded="isModeMenuOpen"
              @click="toggleModeMenu">
              <span>{{ selectedSceneLabel }}</span>
              <img src="/ChevronDown.svg" alt="" class="scene-button-icon" :class="{ 'is-open': isModeMenuOpen }"
                aria-hidden="true" />
            </button>

            <div v-if="isModeMenuOpen" class="scene-menu">
              <button type="button" class="scene-menu-item" :class="{ 'is-active': selectedScene === 'disk_cleanup' }"
                @click="selectMode('disk_cleanup')">
                <img src="/DiskCleanup.svg" alt="" class="scene-menu-icon" aria-hidden="true" />
                <span class="scene-menu-label">清理模式</span>
              </button>
            </div>

            <button type="button" class="submit-button" :disabled="isSubmitting || !canSubmit" @click="submitEntry">
              <img src="/send.svg" alt="" class="submit-icon" aria-hidden="true" />
            </button>
          </div>
        </div>
      </div>

      <div class="quick-actions">
        <button v-for="prompt in quickPrompts" :key="prompt" type="button" class="quick-action-chip"
          :disabled="isSubmitting" @click="applyQuickPrompt(prompt)">
          {{ prompt }}
        </button>
      </div>

      <div class="entry-tip">发送后会先创建新的历史上下文，再进入聊天页继续回复。</div>
    </section>
  </div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { computed, nextTick, onMounted, ref } from 'vue'
import { useRouter } from 'vue-router'

import { AgentHistoryStore } from '../composables/useAgentHistory'
import { pushNotice } from '../composables/useNoticeCenter'

const router = useRouter()
const agentHistoryStore = new AgentHistoryStore()

const inputText = ref('')
const textareaRef = ref<HTMLTextAreaElement | null>(null)
const isSubmitting = ref(false)
const isModeMenuOpen = ref(false)
const selectedScene = ref('disk_cleanup')
const minTextareaHeightRem = 3
const maxTextareaHeightRem = 12

const quickPrompts = [
  '帮我分析当前磁盘占用最大的目录',
  '帮我找出可以安全清理的大文件',
  '先扫描 Downloads 和 Desktop 的占用情况',
]

const canSubmit = computed(() => inputText.value.trim().length > 0)
const selectedSceneLabel = computed(() => selectedScene.value === 'disk_cleanup' ? '清理模式' : '选择模式')

const toggleModeMenu = () => {
  isModeMenuOpen.value = !isModeMenuOpen.value
}

const loadCurrentScene = async () => {
  selectedScene.value = await invoke<string>('get_current_scene')
}

const selectMode = async (scene: string) => {
  if (selectedScene.value !== scene) {
    await invoke('switch_current_scene', { scene })
    selectedScene.value = scene
  }

  isModeMenuOpen.value = false
}

const resizeTextarea = () => {
  const el = textareaRef.value
  if (!el) return

  const rootFontSize = Number.parseFloat(getComputedStyle(document.documentElement).fontSize) || 16
  const minTextareaHeightPx = minTextareaHeightRem * rootFontSize
  const maxTextareaHeightPx = maxTextareaHeightRem * rootFontSize

  el.style.height = 'auto'
  const nextHeightPx = Math.max(minTextareaHeightPx, Math.min(el.scrollHeight, maxTextareaHeightPx))
  el.style.height = `${nextHeightPx / rootFontSize}rem`
  el.style.overflowY = el.scrollHeight > maxTextareaHeightPx ? 'auto' : 'hidden'
}

const resetTextarea = async () => {
  await nextTick()
  resizeTextarea()
}

const applyQuickPrompt = async (prompt: string) => {
  inputText.value = prompt
  await resetTextarea()
  textareaRef.value?.focus()
}

const submitEntry = async () => {
  const content = inputText.value.trim()

  if (!content || isSubmitting.value) {
    return
  }

  isSubmitting.value = true

  try {
    await invoke<string>('create_history_context')
    agentHistoryStore.reset([])
    await router.push('/')
    await invoke('chat', { content })
    inputText.value = ''
    await resetTextarea()
  }
  catch (error) {
    const message = error instanceof Error ? error.message : String(error)
    pushNotice('error', `发起聊天失败：${message}`)
  }
  finally {
    isSubmitting.value = false
  }
}

onMounted(() => {
  nextTick(() => {
    resizeTextarea()
    textareaRef.value?.focus()
  })
  loadCurrentScene()
})
</script>

<style scoped>
.new-chat-view {
  height: 100%;
  min-height: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 2rem 2rem 4rem;
  box-sizing: border-box;
  background: #f9fbfc;
}

.entry-shell {
  width: 100%;
  max-width: 88rem;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 1.75rem;
}

.brand-block {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.75rem;
}

.brand-logo {
  width: 3rem;
  height: 3rem;
  object-fit: contain;
}

.brand-title {
  font-size: 3rem;
  line-height: 1;
  font-weight: 700;
  color: #151515;
  letter-spacing: -0.03em;
}

.entry-copy {
  font-size: 2.25rem;
  line-height: 1.2;
  font-weight: 500;
  color: #242424;
  text-align: center;
}

.entry-composer {
  width: 100%;
  display: flex;
  justify-content: center;
}

.entry-input-shell {
  width: min(100%, 92rem);
  display: flex;
  align-items: center;
  gap: 0.375rem;
  padding: 0.3rem 1.2rem;
  border-radius: 1.6rem;
  background: #ffffff;
  border: 0.0625rem solid #e4e7eb;
  box-shadow: 0 0.125rem 0.5rem rgba(15, 23, 42, 0.06);
}

.entry-textarea {
  flex: 1;
  min-height: 3rem;
  max-height: 12rem;
  padding: 0.75rem 0;
  box-sizing: border-box;
  border: 0;
  background: #ffffff;
  color: #2a2a2a;
  font-size: 1rem;
  line-height: 1.5;
  resize: none;
  outline: none;
  overflow-y: auto;
}

.entry-textarea::placeholder {
  color: #7d8794;
}

.entry-textarea:disabled {
  background: #ffffff;
  color: #9aa1aa;
}

.entry-actions {
  position: relative;
  display: flex;
  gap: 0.125rem;
  align-items: center;
  align-self: center;
}

.scene-button {
  height: 2.375rem;
  padding: 0 0.75rem;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 0.375rem;
  border: 0;
  background: transparent;
  color: #1f1f1f;
  font-size: 0.8125rem;
  line-height: 1;
  font-weight: 600;
  cursor: pointer;
  white-space: nowrap;
  border-radius: 999px;
}

.scene-button:disabled {
  cursor: not-allowed;
  opacity: 0.6;
}

.scene-button-icon {
  width: 0.75rem;
  height: 0.75rem;
  transition: transform 0.2s ease;
}

.scene-button-icon.is-open {
  transform: rotate(180deg);
}

.scene-menu {
  position: absolute;
  right: 4rem;
  bottom: calc(100% + 0.625rem);
  min-width: 10.5rem;
  padding: 0.5rem;
  border-radius: 1rem;
  border: 0.0625rem solid #e4e7eb;
  background: #ffffff;
  box-shadow: 0 0.75rem 2rem rgba(15, 23, 42, 0.12);
}

.scene-menu-item {
  width: 100%;
  display: flex;
  align-items: center;
  gap: 0.625rem;
  padding: 0.75rem 0.875rem;
  border: 0;
  border-radius: 0.75rem;
  background: transparent;
  cursor: pointer;
  color: #1f1f1f;
  font-size: 0.875rem;
  text-align: left;
}

.scene-menu-item.is-active {
  background: #f4f7f9;
}

.scene-menu-icon {
  width: 1rem;
  height: 1rem;
}

.scene-menu-label {
  white-space: nowrap;
}

.quick-action-chip {
  border: 0.0625rem solid #e3e6ea;
  background: #ffffff;
  color: #525866;
  border-radius: 999px;
  padding: 0.75rem 1.125rem;
  font-size: 0.9375rem;
  line-height: 1.4;
  cursor: pointer;
  transition: background-color 0.2s ease, border-color 0.2s ease;
}

.quick-action-chip:hover:not(:disabled) {
  background: #f7f8fa;
  border-color: #d5dbe3;
}

.quick-action-chip:disabled {
  cursor: not-allowed;
  opacity: 0.6;
}

.quick-actions {
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
  gap: 0.875rem;
  width: min(100%, 72rem);
}

.entry-tip {
  font-size: 0.9375rem;
  line-height: 1.6;
  color: #8c95a2;
  text-align: center;
}

.submit-button {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 2.375rem;
  height: 2.375rem;
  flex-shrink: 0;
  border: 0;
  border-radius: 999px;
  background: #171717;
  color: #ffffff;
  cursor: pointer;
  transition: transform 0.16s ease, opacity 0.2s ease, background-color 0.2s ease;
}

.submit-button:hover:not(:disabled) {
  background: #000000;
}

.submit-button:disabled {
  cursor: not-allowed;
  opacity: 0.6;
}

.submit-icon {
  width: 0.8rem;
  height: 0.8rem;
}
</style>
