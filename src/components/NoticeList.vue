<template>
  <div v-if="notices.length" class="notice-list-wrap">
    <transition-group name="notice-fade" tag="div" class="notice-list">
      <div
        v-for="item in notices"
        :key="item.id"
        class="notice-item"
        :class="`notice-item--${item.type}`"
      >
        <span class="notice-text">{{ item.text }}</span>
      </div>
    </transition-group>
  </div>
</template>

<script setup lang="ts">
import { useNoticeCenter } from '../composables/useNoticeCenter'

const { notices } = useNoticeCenter()
</script>

<style scoped>
.notice-list-wrap {
  position: absolute;
  left: 50%;
  bottom: 5.75rem;
  transform: translateX(-50%);
  width: min(36rem, calc(100% - 2rem));
  pointer-events: none;
  z-index: 30;
}

.notice-list {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.notice-item {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.625rem 0.75rem;
  border-radius: 0.625rem;
  border: 0.0625rem solid #e5eceb;
  box-shadow: 0 0.1875rem 0.625rem rgba(25, 40, 36, 0.05);
  backdrop-filter: blur(0.25rem);
  background: #f8fbfa;
}

.notice-item--success {
  border-color: #d8e9dd;
  background: #f4faf5;
}

.notice-item--warning {
  border-color: #eee5cd;
  background: #fcf9f1;
}

.notice-item--error {
  border-color: #eddad8;
  background: #fcf5f4;
}

.notice-text {
  color: #4d5b59;
  font-size: 0.8125rem;
  line-height: 1.4;
  word-break: break-word;
}

.notice-fade-enter-active,
.notice-fade-leave-active {
  transition: all 0.22s ease;
}

.notice-fade-enter-from,
.notice-fade-leave-to {
  opacity: 0;
  transform: translateY(0.375rem);
}
</style>

