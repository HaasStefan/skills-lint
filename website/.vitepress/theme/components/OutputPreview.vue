<script setup lang="ts">
import TerminalWindow from './TerminalWindow.vue'
</script>

<template>
  <div class="home-section">
    <TerminalWindow title="skills-lint">
      <pre class="output-pre"><span class="ansi-bold">File                                    Model      Tokens   Warning   Error   Status</span>
<span class="ansi-dim">────────────────────────────────────────────────────────────────────────────────────────</span>
.github/skills/code-review/SKILL.md     opus-4.5    <span class="ansi-green">4,821</span>   <span class="ansi-dim">  8,000</span>  <span class="ansi-dim"> 12,000</span>   <span class="ansi-green">✓ PASS</span>
  <span class="ansi-dim">┆</span>                                     gpt-4o      <span class="ansi-green">5,102</span>   <span class="ansi-dim">  8,000</span>  <span class="ansi-dim"> 12,000</span>   <span class="ansi-green">✓ PASS</span>
.github/skills/deploy/SKILL.md          opus-4.5    <span class="ansi-yellow ansi-bold">9,340</span>   <span class="ansi-dim">  8,000</span>  <span class="ansi-dim"> 12,000</span>   <span class="ansi-yellow ansi-bold">⚠ WARN</span>
  <span class="ansi-dim">┆</span>                                     gpt-4o      <span class="ansi-yellow ansi-bold">9,812</span>   <span class="ansi-dim">  8,000</span>  <span class="ansi-dim"> 12,000</span>   <span class="ansi-yellow ansi-bold">⚠ WARN</span>

  <span class="ansi-bold">Results:</span> <span class="ansi-green">2 passed</span>, <span class="ansi-yellow">2 warnings</span> across 2 files</pre>
    </TerminalWindow>
  </div>
</template>

<style scoped>
.output-pre {
  margin: 0;
  padding: 0;
  font-family: inherit;
  font-size: 0.8rem;
  line-height: 1.6;
  white-space: pre;
  overflow-x: auto;
  background: transparent;
  color: #c0c0d0;
  border: none;
}
</style>
