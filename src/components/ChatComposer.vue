<template>
  <div class="bottom-section">
    <div class="input-row">
      <textarea
        ref="textareaRef"
        v-model="inputText"
        rows="1"
        class="main-input main-textarea"
        placeholder="请输入内容"
        @input="resizeTextarea"
        @keydown.enter.exact.prevent="submitChat"
      />
      <button
        type="button"
        class="mode-trigger"
        aria-label="模式选择"
        :aria-expanded="isModeMenuOpen"
        @click="toggleModeMenu"
      >
        <span class="mode-label">清理模式</span>
        <img
          src="/ChevronDown.svg"
          alt=""
          class="mode-caret-icon"
          :class="{ 'is-open': isModeMenuOpen }"
          aria-hidden="true"
        />
      </button>
      <div v-if="isModeMenuOpen" class="mode-menu">
        <button type="button" class="mode-menu-item is-active" @click="selectMode('disk_cleanup')">
          <img src="/DiskCleanup.svg" alt="清理模式" class="mode-menu-icon" />
          <span class="mode-menu-text-group">
            <span class="mode-menu-title">清理模式</span>
            <span class="mode-menu-desc">磁盘清理与空间分析</span>
          </span>
          <span class="mode-menu-check">✓</span>
        </button>
      </div>
      <button
        type="button"
        class="send-button"
        aria-label="发送"
        :disabled="isSending"
        @click="submitChat"
      >
        <img
          src="/send.svg"
          alt="发送"
          class="send-icon"
        />
      </button>
    </div>
    <span class="tip-text">AI不一定准确，请仔细查看以防万一</span>
  </div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { nextTick, onMounted, ref } from 'vue'

import { pushNotice } from '../composables/useNoticeCenter'

const inputText = ref('')
const textareaRef = ref<HTMLTextAreaElement | null>(null)
const isSending = ref(false)
const isModeMenuOpen = ref(false)
const selectedScene = ref('disk_cleanup')
const maxTextareaHeightRem = 11.25

const toggleModeMenu = () => {
  isModeMenuOpen.value = !isModeMenuOpen.value
}

const loadCurrentScene = async () => {
  selectedScene.value = await invoke<string>('get_current_scene')
}

// 设置当前场景后，自动关闭菜单
const selectMode = async (scene: string) => {
  if (selectedScene.value === scene) {
    isModeMenuOpen.value = false
    return
  }

  await invoke('switch_current_scene', { scene })
  selectedScene.value = scene
  isModeMenuOpen.value = false
}

/// 输入框高度自适应，最大高度为 11.25rem，超过后显示滚动条
const resizeTextarea = () => {
  const el = textareaRef.value
  if (!el) return

  const rootFontSize = Number.parseFloat(
    getComputedStyle(document.documentElement).fontSize,
  ) || 16
  const maxTextareaHeightPx = maxTextareaHeightRem * rootFontSize

  el.style.height = 'auto'
  const nextHeightPx = Math.min(el.scrollHeight, maxTextareaHeightPx)
  const nextHeightRem = nextHeightPx / rootFontSize
  el.style.height = `${nextHeightRem}rem`
  el.style.overflowY = el.scrollHeight > maxTextareaHeightPx ? 'auto' : 'hidden'
}

const resetTextarea = async () => {
  await nextTick()
  resizeTextarea()
}

const submitChat = async () => {
  const content = inputText.value.trim()

  if (!content || isSending.value) {
    return
  }

  isSending.value = true

  try {
    await invoke('chat', { content })
    inputText.value = ''
    await resetTextarea()
  }
  catch (error) {
    const message = error instanceof Error ? error.message : String(error)
    pushNotice('error', `发送失败：${message}`)
  }
  finally {
    isSending.value = false
  }
}

onMounted(() => {
  nextTick(() => {
    resizeTextarea()
  })
  loadCurrentScene()
})
</script>

<style scoped>
.main-input {
  display: block;
  width: 100%;
  background-color: #ffffff;
  border: 0.0625rem solid #e5e7eb;
  border-radius: 0.75rem;
  padding: 0.625rem 0.75rem;
  font-size: 0.875rem;
  outline: none;
  box-sizing: border-box;
}

.main-textarea {
  flex: 1;
  width: auto;
  min-height: 2.625rem;
  max-height: 11.25rem;
  line-height: 1.5;
  resize: none;
  overflow-y: hidden;
}

.input-row {
  position: relative;
  display: flex;
  align-items: flex-end;
  gap: 0.5rem;
}

.mode-trigger {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 0.5rem;
  height: 2.625rem;
  min-width: 6.5rem;
  padding: 0 0.875rem;
  border: 0.0625rem solid #e2e8f0;
  border-radius: 0.75rem;
  background-color: #ffffff;
  cursor: pointer;
  color: #5f6b7a;
  font-size: 0.875rem;
  font-weight: 400;
  line-height: 1.5;
  outline: none;
  box-sizing: border-box;
  transition: background-color 0.2s ease, border-color 0.2s ease;
}

.mode-trigger:hover {
  background-color: #f3f6fa;
  border-color: #cdd8e5;
}

.mode-trigger:focus-visible {
  outline: 0.125rem solid #c7d2fe;
  outline-offset: 0.125rem;
}

.mode-label {
  font-size: inherit;
  font-weight: inherit;
  line-height: inherit;
  white-space: nowrap;
}

.mode-caret-icon {
  width: 0.75rem;
  height: 0.75rem;
  display: block;
  opacity: 0.72;
  flex-shrink: 0;
  transition: transform 0.2s ease, opacity 0.2s ease;
}

.mode-caret-icon.is-open {
  opacity: 1;
  transform: rotate(180deg);
}

.mode-menu {
  position: absolute;
  right: 3.25rem;
  bottom: calc(100% + 0.5rem);
  width: 16rem;
  padding: 0.5rem;
  border-radius: 1rem;
  background-color: #ffffff;
  border: 0.0625rem solid #e5e7eb;
  box-shadow: 0 0.75rem 2rem rgba(15, 23, 42, 0.14);
  z-index: 10;
}

.mode-menu-item {
  width: 100%;
  border: 0;
  background: transparent;
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.4rem;
  border-radius: 0.875rem;
  text-align: left;
  cursor: pointer;
}

.mode-menu-item.is-active {
  background-color: #f8fafc;
}

.mode-menu-icon {
  width:  1.2rem;
  height: 1.2rem;
  flex-shrink: 0;
}

.mode-menu-text-group {
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 0.125rem;
}

.mode-menu-title {
  font-size: 0.875rem;
  font-weight: 400;
  line-height: 1.5;
  color: #5f6b7a;
}

.mode-menu-desc {
  font-size: 0.8125rem;
  color: #6b7280;
}

.mode-menu-check {
  margin-left: auto;
  color: #111827;
  font-weight: 500;
}

.send-button {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 2.625rem;
  height: 2.625rem;
  border: 0.0625rem solid #e2e8f0;
  border-radius: 0.75rem;
  background-color: #ffffff;
  cursor: pointer;
  transition: background-color 0.2s ease, border-color 0.2s ease, transform 0.12s ease;
}

.send-button:disabled {
  cursor: not-allowed;
  opacity: 0.6;
}

.send-button:hover {
  background-color: #f3f6fa;
  border-color: #cdd8e5;
}

.send-button:active {
  transform: scale(0.96);
  background-color: #eaf1f8;
}

.send-button:focus-visible {
  outline: 0.125rem solid #c7d2fe;
  outline-offset: 0.125rem;
}

.send-icon {
  width: 1.125rem;
  height: 1.125rem;
  display: block;
}

.main-input:focus {
  border-color: #c7d2fe;
}

.bottom-section {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  background-color: #f9fbfc;
  padding: 0.625rem 0.375rem;
  border-top: 0.0625rem solid #d1dbe5;
}

.tip-text {
  display: block;
  text-align: center;
  font-size: 0.75rem;
  color: #c8c8c8;
  line-height: 1.5;
}
</style>
