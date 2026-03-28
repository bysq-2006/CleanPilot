<template>
  <div class="common-frame-box" :style="boxStyle">
    <slot />
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

const props = withDefaults(defineProps<{
  maxHeight?: string
}>(), {
  maxHeight: '14rem',
})

const boxStyle = computed(() => ({
  maxHeight: props.maxHeight,
}))
</script>

<style scoped>
.common-frame-box {
  max-width: min(42rem, 100%);
  padding: 0.6rem 0.8rem;
  border-radius: 0.875rem;
  border: 0.07rem solid #d2d9e0;
  overflow: auto;
}
</style>
