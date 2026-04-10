<template>
  <Transition name="ghost-blanket-pop">
      <div v-if="modelValue" class="ghost-blanket-overlay" @click.self="emit('close')">
        <div class="ghost-blanket-card">
          <button type="button" class="ghost-blanket-close" @click="emit('close')" aria-label="关闭">
            ×
          </button>

        <div class="ghost-blanket-card-body">
          <GhostBlanketDiskCleanupList
            v-if="type === 'disk_cleanup'"
            :record-path="modelValue!"
          />
          <div v-else class="ghost-blanket-placeholder">暂不支持该类型的渲染</div>
        </div>
      </div>
    </div>
  </Transition>
</template>

<script setup lang="ts">
import GhostBlanketDiskCleanupList from './GhostBlanketDiskCleanupList.vue'

defineProps<{
  modelValue: string | null
  type: string | null
}>()

const emit = defineEmits<{
  close: []
}>()
</script>

<style scoped>
.ghost-blanket-overlay {
  position: absolute;
  inset: 0;
  z-index: 1000;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 2rem;
  background: rgba(246, 249, 248, 0.24);
  backdrop-filter: blur(18px);
  -webkit-backdrop-filter: blur(18px);
}

.ghost-blanket-pop-enter-active,
.ghost-blanket-pop-leave-active {
  transition: opacity 0.24s ease;
}

.ghost-blanket-pop-enter-active .ghost-blanket-card,
.ghost-blanket-pop-leave-active .ghost-blanket-card {
  transition: transform 0.28s cubic-bezier(0.22, 1, 0.36, 1), opacity 0.24s ease;
  transform-origin: center center;
}

.ghost-blanket-pop-enter-from,
.ghost-blanket-pop-leave-to {
  opacity: 0;
}

.ghost-blanket-pop-enter-from .ghost-blanket-card,
.ghost-blanket-pop-leave-to .ghost-blanket-card {
  opacity: 0;
  transform: scale(0.82);
}

.ghost-blanket-pop-enter-to .ghost-blanket-card,
.ghost-blanket-pop-leave-from .ghost-blanket-card {
  opacity: 1;
  transform: scale(1);
}

.ghost-blanket-card {
  position: relative;
  width: 90%;
  height: 95%;
  border-radius: 1.5rem;
  border: 1px solid rgba(255, 255, 255, 0.72);
  background: rgba(255, 255, 255, 0.9);
  box-shadow: 0 1.5rem 4rem rgba(31, 42, 40, 0.12);
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.ghost-blanket-close {
  position: absolute;
  top: 1rem;
  right: 1rem;
  width: 1.75rem;
  height: 1.75rem;
  border: 0;
  border-radius: 999px;
  background: #e34d4d;
  color: #ffffff;
  font-size: 0.95rem;
  line-height: 1;
  cursor: pointer;
  display: inline-flex;
  align-items: center;
  justify-content: center;
}

.ghost-blanket-close:hover {
  background: #cf3f3f;
}

.ghost-blanket-card-body {
  flex: 1;
  min-height: 0;
  padding: 3.5rem 1.5rem 1.5rem;
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.ghost-blanket-placeholder {
  flex: 1;
  border-radius: 1rem;
  border: 1px dashed rgba(184, 196, 191, 0.92);
  background: linear-gradient(180deg, rgba(248, 251, 250, 0.92), rgba(255, 255, 255, 0.96));
  display: flex;
  align-items: center;
  justify-content: center;
  text-align: center;
  padding: 2rem;
  color: #647874;
}

.ghost-blanket-placeholder-error {
  color: #bf4b4b;
}

.ghost-blanket-record {
  flex: 1;
  min-height: 0;
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

</style>
