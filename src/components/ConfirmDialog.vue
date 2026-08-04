<!-- 提供统一的“是/否”操作确认弹窗。 -->
<template>
  <Teleport to="body">
    <Transition name="confirm-fade">
      <div v-if="open" class="confirm-mask" @click.self="cancel">
        <section class="confirm-card" role="alertdialog" aria-modal="true">
          <h3>{{ title }}</h3>
          <p>{{ message }}</p>
          <code v-if="detail">{{ detail }}</code>

          <div class="confirm-actions">
            <button type="button" class="confirm-no" @click="cancel">否</button>
            <button type="button" class="confirm-yes" @click="emit('confirm')">是</button>
          </div>
        </section>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
defineProps<{
  open: boolean
  title: string
  message: string
  detail?: string
}>()

const emit = defineEmits<{
  confirm: []
  cancel: []
}>()

const cancel = () => emit('cancel')
</script>

<style scoped>
.confirm-mask {
  position: fixed;
  inset: 0;
  z-index: 1000;
  display: grid;
  place-items: center;
  padding: 1.5rem;
  background: rgba(30, 40, 37, 0.22);
  backdrop-filter: blur(0.25rem);
}

.confirm-card {
  width: min(25rem, 100%);
  padding: 1.5rem;
  border: 1px solid rgba(224, 231, 228, 0.95);
  border-radius: 1.25rem;
  background: #fff;
  box-shadow: 0 1.5rem 4rem rgba(45, 63, 57, 0.18);
}

h3,
p {
  margin: 0;
}

h3 {
  color: #243230;
  font-size: 1.05rem;
}

p {
  margin-top: 0.65rem;
  color: #627571;
  line-height: 1.6;
}

code {
  display: block;
  margin-top: 0.75rem;
  padding: 0.7rem;
  overflow-wrap: anywhere;
  border-radius: 0.7rem;
  background: #f5f8f7;
  color: #40514d;
  font-family: inherit;
}

.confirm-actions {
  display: flex;
  justify-content: flex-end;
  gap: 0.625rem;
  margin-top: 1.25rem;
}

button {
  min-width: 4.5rem;
  border: 0;
  border-radius: 0.75rem;
  padding: 0.65rem 1rem;
  font-weight: 600;
  cursor: pointer;
}

.confirm-no {
  background: rgba(37, 51, 48, 0.08);
  color: #314341;
}

.confirm-yes {
  background: rgba(227, 77, 77, 0.14);
  color: #c13f3f;
}

.confirm-fade-enter-active,
.confirm-fade-leave-active {
  transition: opacity 0.16s ease;
}

.confirm-fade-enter-from,
.confirm-fade-leave-to {
  opacity: 0;
}
</style>
