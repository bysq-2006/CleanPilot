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
const maxTextareaHeightRem = 11.25

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
  display: flex;
  align-items: flex-end;
  gap: 0.5rem;
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
