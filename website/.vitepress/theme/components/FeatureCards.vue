<script setup lang="ts">
const features = [
  {
    title: 'Multi-Model',
    description: 'Lint against gpt-4o, gpt-4, gpt-3.5-turbo and more. Each model uses its correct tiktoken encoding.',
  },
  {
    title: '6 Platforms',
    description: 'Pre-built binaries for macOS, Linux, and Windows — ARM64 and x64.',
  },
  {
    title: 'CI-Friendly',
    description: 'Exit codes 0/1/2/3 for pass/error/warning/runtime. Use --quiet in pipelines.',
  },
  {
    title: 'Overrides',
    description: 'Per-file thresholds. Give specific skills a larger budget without changing global limits.',
  },
  {
    title: 'Rust + tiktoken',
    description: 'Tokenizes with tiktoken-rs. Handles hundreds of files in milliseconds.',
  },
  {
    title: 'npm Install',
    description: 'npm install -g and go. Optional deps pick the right binary for your platform.',
  },
]
</script>

<template>
  <div class="home-section">
    <div class="feature-list">
      <div v-for="(f, i) in features" :key="i" class="feature-item">
        <h3>{{ f.title }}</h3>
        <p>{{ f.description }}</p>
      </div>
    </div>
  </div>
</template>
