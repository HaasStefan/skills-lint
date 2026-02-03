<script setup lang="ts">
import AsciiHero from './AsciiHero.vue'
import InstallBlock from './InstallBlock.vue'
import OutputPreview from './OutputPreview.vue'
import FeatureCards from './FeatureCards.vue'
</script>

<template>
  <div class="home-terminal">
    <AsciiHero />
    <InstallBlock />
    <OutputPreview />
    <FeatureCards />
  </div>
</template>

<style scoped>
.home-terminal {
  margin: 0 auto;
  width: 100%;
}
</style>
