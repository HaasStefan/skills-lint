<script setup lang="ts">
const features = [
  {
    title: 'Multi-Model Budgets',
    description: 'Define per-model token limits with different encodings. Compare how skill files tokenize across Claude, GPT, and more.',
  },
  {
    title: '6 Platform Binaries',
    description: 'Pre-built native binaries for macOS (ARM64, x64), Linux (x64, ARM64), and Windows (x64, ARM64).',
  },
  {
    title: 'CI-Friendly',
    description: 'Distinct exit codes for pass, warning, and error. Use --quiet to suppress the banner in automated pipelines.',
  },
  {
    title: 'Override Support',
    description: 'Apply different thresholds to specific files. Grant larger budgets to complex skills without loosening global limits.',
  },
  {
    title: 'Fast Rust Core',
    description: 'Built in Rust with tiktoken encodings. Tokenize hundreds of files in milliseconds.',
  },
  {
    title: 'npm Distribution',
    description: 'Install globally via npm. Platform-specific optional dependencies auto-select the right binary.',
  },
]
</script>

<template>
  <div class="home-section">
    <div class="feature-grid">
      <div v-for="(f, i) in features" :key="i" class="feature-card">
        <h3>{{ f.title }}</h3>
        <p>{{ f.description }}</p>
      </div>
    </div>
  </div>
</template>
