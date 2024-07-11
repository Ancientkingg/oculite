<script setup lang="ts">
import { ref } from 'vue';

const msg = ref('');
</script>

<template>
  <main>
    <h1>Playground</h1>
    <InputText v-model="msg" />
    <p>{{ msg }}</p>
  </main>
</template>

<style scoped></style>