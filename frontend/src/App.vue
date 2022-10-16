<script setup lang="ts">
import { init_wasm, log_weather_data } from '../pkg'
</script>

<template>
<h1>Hey</h1>
</template>

<script lang="ts">
init_wasm();
await log_weather_data();
</script>