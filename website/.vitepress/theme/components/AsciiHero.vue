<script setup lang="ts">
import { withBase } from 'vitepress'

const banner = `███████╗██╗  ██╗██╗██╗     ██╗     ███████╗      ██╗     ██╗███╗   ██╗████████╗
██╔════╝██║ ██╔╝██║██║     ██║     ██╔════╝      ██║     ██║████╗  ██║╚══██╔══╝
███████╗█████╔╝ ██║██║     ██║     ███████╗█████╗██║     ██║██╔██╗ ██║   ██║
╚════██║██╔═██╗ ██║██║     ██║     ╚════██║╚════╝██║     ██║██║╚██╗██║   ██║
███████║██║  ██╗██║███████╗███████╗███████║      ███████╗██║██║ ╚████║   ██║
╚══════╝╚═╝  ╚═╝╚═╝╚══════╝╚══════╝╚══════╝      ╚══════╝╚═╝╚═╝  ╚═══╝   ╚═╝`
</script>

<template>
  <div class="ascii-hero">
    <div class="ascii-banner">
      <pre>{{ banner }}</pre>
      <h1 class="ascii-fallback">skills-lint</h1>
    </div>
    <p class="ascii-tagline">Token budget linter for agent skill files</p>
    <div class="cta-group">
      <a :href="withBase('/guide/')" class="cta-btn cta-btn--primary">Get Started</a>
      <a href="https://github.com/HaasStefan/skills-lint" class="cta-btn cta-btn--secondary" target="_blank" rel="noopener">GitHub</a>
    </div>
  </div>
</template>

<style scoped>
.ascii-hero {
  text-align: center;
  padding: 60px 24px 40px;
}

.ascii-banner {
  margin-bottom: 16px;
}

.ascii-banner pre {
  margin: 0 auto;
  display: inline-block;
  text-align: left;
  white-space: pre;
}

.ascii-tagline {
  color: var(--vp-c-text-3);
  font-size: 1.1rem;
  margin: 0 0 32px;
}

.cta-group {
  justify-content: center;
}
</style>
