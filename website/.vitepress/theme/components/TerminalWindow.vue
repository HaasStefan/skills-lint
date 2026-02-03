<script setup lang="ts">
defineProps<{
  title?: string
}>()
</script>

<template>
  <div class="terminal-window">
    <div class="terminal-titlebar">
      <span class="terminal-dot terminal-dot--red"></span>
      <span class="terminal-dot terminal-dot--yellow"></span>
      <span class="terminal-dot terminal-dot--green"></span>
      <span v-if="title" class="terminal-title">{{ title }}</span>
    </div>
    <div class="terminal-body">
      <slot />
    </div>
  </div>
</template>
