<template>
  <div id="app">
    <router-view />
  </div>
</template>


<script setup>
import { useAppStore, useAuthStore, useProxyStore } from './stores'
import { onMounted } from 'vue'

onMounted(async () => {
  const appStore = useAppStore()
  const authStore = useAuthStore()
  const proxyStore = useProxyStore()

  try {
    // 并行初始化所有 store
    await Promise.all([
      appStore.initialize(),
      authStore.initialize(),
      proxyStore.initialize()
    ])
    console.log('🎉 所有 Store 初始化完成')
  } catch (error) {
    console.error('❌ Store 初始化失败:', error)
    // 如果初始化失败，可以显示错误通知或重试
  }
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
