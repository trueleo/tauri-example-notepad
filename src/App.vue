<script setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Text from "./components/Text.vue"
</script>

<template>
  <Suspense>
    <Text />
    <template #fallback>
      .. Loading
    </template>
  </Suspense>
</template>

<style scoped>
* {
  max-width: none;
}
</style>
