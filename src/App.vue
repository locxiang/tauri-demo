<template>
  <div id="app">
    <router-view />
  </div>
</template>


<script setup>
import { useAppStore, useAuthStore, useProxyStore } from './stores'
import { onMounted } from 'vue'

onMounted(() => {
  const appStore = useAppStore()
  const authStore = useAuthStore()
  const proxyStore = useProxyStore()

  appStore.initialize()
  authStore.initialize()
  proxyStore.initialize()
})


</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  width: 100%;
  height: 100vh;
  margin: 0;
  padding: 0;
}

* {
  box-sizing: border-box;
}

body, html {
  margin: 0;
  padding: 0;
  width: 100%;
  height: 100%;
  overflow: hidden;
}
</style>
