<script setup lang="ts">
import TerminalWindow from './TerminalWindow.vue'
</script>

<template>
  <div class="home-section">
    <TerminalWindow title="skills-lint">
      <pre class="output-pre">  <span class="ansi-bold">.github/skills/code-review/SKILL.md</span>
  <span class="ansi-dim">├─</span> <span class="ansi-green">skill-structure</span>   valid   <span class="ansi-green">✓ PASS</span>
  <span class="ansi-dim">│</span>
  <span class="ansi-dim">└─</span> <span class="ansi-yellow ansi-bold">token-limit</span>
     ╭─────────┬────────┬─────────┬────────┬────────╮
     │ <span class="ansi-bold">Model</span>   ┆ <span class="ansi-bold">Tokens</span> ┆ <span class="ansi-bold">Warning</span> ┆  <span class="ansi-bold">Error</span> ┆ <span class="ansi-bold">Status</span> │
     ╞═════════╪════════╪═════════╪════════╪════════╡
     │ gpt-5   ┆  <span class="ansi-green">4,281</span> ┆ <span class="ansi-dim"> 16,000</span> ┆ <span class="ansi-dim"> 32,000</span> ┆ <span class="ansi-green">✓ PASS</span> │
     │ gpt-4o  ┆  <span class="ansi-green">4,821</span> ┆ <span class="ansi-dim">  8,000</span> ┆ <span class="ansi-dim"> 16,000</span> ┆ <span class="ansi-green">✓ PASS</span> │
     │ gpt-4   ┆  <span class="ansi-yellow ansi-bold">2,340</span> ┆ <span class="ansi-dim">  2,000</span> ┆ <span class="ansi-dim">  4,000</span> ┆ <span class="ansi-yellow ansi-bold">⚠ WARN</span> │
     ╰─────────┴────────┴─────────┴────────┴────────╯

  <span class="ansi-dim">──────────────────────────────────────────────────</span>

  <span class="ansi-bold">.github/skills/deploy/SKILL.md</span>
  <span class="ansi-dim">├─</span> <span class="ansi-green">skill-structure</span>   valid   <span class="ansi-green">✓ PASS</span>
  <span class="ansi-dim">│</span>
  <span class="ansi-dim">└─</span> <span class="ansi-red ansi-bold">token-limit</span>
     ╭─────────┬────────┬─────────┬────────┬─────────╮
     │ <span class="ansi-bold">Model</span>   ┆ <span class="ansi-bold">Tokens</span> ┆ <span class="ansi-bold">Warning</span> ┆  <span class="ansi-bold">Error</span> ┆ <span class="ansi-bold">Status</span>  │
     ╞═════════╪════════╪═════════╪════════╪═════════╡
     │ gpt-5   ┆  <span class="ansi-green">8,710</span> ┆ <span class="ansi-dim"> 16,000</span> ┆ <span class="ansi-dim"> 32,000</span> ┆ <span class="ansi-green">✓ PASS</span>  │
     │ gpt-4o  ┆  <span class="ansi-yellow ansi-bold">9,340</span> ┆ <span class="ansi-dim">  8,000</span> ┆ <span class="ansi-dim"> 16,000</span> ┆ <span class="ansi-yellow ansi-bold">⚠ WARN</span>  │
     │ gpt-4   ┆  <span class="ansi-red ansi-bold">4,812</span> ┆ <span class="ansi-dim">  2,000</span> ┆ <span class="ansi-dim">  4,000</span> ┆ <span class="ansi-red ansi-bold">✗ ERROR</span> │
     ╰─────────┴────────┴─────────┴────────┴─────────╯

  <span class="ansi-dim">──────────────────────────────────────────────────</span>

  <span class="ansi-bold">(skill index)</span>
  <span class="ansi-dim">└─</span> <span class="ansi-green">skill-index-budget</span>
     ╭────────┬────────┬─────────┬───────┬────────╮
     │ <span class="ansi-bold">Model</span>  ┆ <span class="ansi-bold">Tokens</span> ┆ <span class="ansi-bold">Warning</span> ┆ <span class="ansi-bold">Error</span> ┆ <span class="ansi-bold">Status</span> │
     ╞════════╪════════╪═════════╪═══════╪════════╡
     │ gpt-4o ┆    <span class="ansi-green">312</span> ┆ <span class="ansi-dim">  2,000</span> ┆ <span class="ansi-dim"> 4,000</span> ┆ <span class="ansi-green">✓ PASS</span> │
     │ gpt-4  ┆    <span class="ansi-green">312</span> ┆ <span class="ansi-dim">  1,000</span> ┆ <span class="ansi-dim"> 2,000</span> ┆ <span class="ansi-green">✓ PASS</span> │
     ╰────────┴────────┴─────────┴───────┴────────╯

  <span class="ansi-bold">Results:</span> <span class="ansi-green">9 passed</span>, <span class="ansi-yellow">2 warnings</span>, <span class="ansi-red ansi-bold">1 error</span> across 2 files</pre>
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
