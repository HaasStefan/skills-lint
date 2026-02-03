<script setup lang="ts">
import { ref } from 'vue'
import TerminalWindow from './TerminalWindow.vue'

const copied = ref(false)
const command = 'npm install -g @haasstefan/skills-lint'

function copy() {
  navigator.clipboard.writeText(command)
  copied.value = true
  setTimeout(() => { copied.value = false }, 2000)
}
</script>

<template>
  <div class="home-section">
    <TerminalWindow title="Terminal">
      <div class="install-line">
        <span class="ansi-dim">$</span>
        <span class="install-cmd">{{ command }}</span>
        <button class="copy-btn" @click="copy">{{ copied ? 'Copied!' : 'Copy' }}</button>
      </div>
    </TerminalWindow>
  </div>
</template>

<style scoped>
.install-line {
  display: flex;
  align-items: center;
  gap: 10px;
}

.install-cmd {
  flex: 1;
  color: #c0c0d0;
}

.copy-btn {
  flex-shrink: 0;
}
</style>
