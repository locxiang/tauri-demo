<script setup lang="ts">
import { computed } from 'vue';
import { useProxyStore } from '../../../stores/proxyStore';

const proxyStore = useProxyStore();

// 计算属性用于显示过滤状态
const activeFiltersCount = computed(() => {
  const filters = proxyStore.filters;
  let count = 0;

  if (filters.srcIp) count++;
  if (filters.dstIp) count++;
  if (filters.url) count++;

  return count;
});

const updateFilterValue = (key: string, value: string) => {
  proxyStore.updateFilter(key as any, value);
};
</script>

<template>
  <div class="bg-gradient-to-br from-slate-900/60 to-slate-800/40 backdrop-blur-lg border border-blue-500/10 rounded-lg p-3 mb-4">
    <!-- 过滤器标题和开关 -->
    <div class="flex items-center justify-between mb-3">
      <div class="flex items-center gap-3">
        <div class="text-lg">🔍</div>
        <h3 class="text-lg font-semibold text-slate-200">数据包过滤</h3>
        <div v-if="activeFiltersCount > 0"
             class="flex items-center gap-1 px-2 py-1 bg-blue-500/20 border border-blue-500/30 rounded-full">
          <div class="w-2 h-2 bg-blue-400 rounded-full animate-pulse"></div>
          <span class="text-xs text-blue-300 font-medium">{{ activeFiltersCount }}个条件</span>
        </div>
      </div>

      <div class="flex items-center gap-2">
        <!-- 启用/禁用过滤器 -->
        <button
          @click="proxyStore.toggleFilter"
          :class="[
            'flex items-center gap-2 px-3 py-1 rounded-md text-sm font-medium transition-all duration-300',
            proxyStore.filters.enabled
              ? 'bg-green-500/20 border border-green-500/30 text-green-300 hover:bg-green-500/30'
              : 'bg-gray-500/20 border border-gray-500/30 text-gray-300 hover:bg-gray-500/30'
          ]"
        >
          <div :class="['w-2 h-2 rounded-full', proxyStore.filters.enabled ? 'bg-green-400' : 'bg-gray-400']"></div>
          {{ proxyStore.filters.enabled ? '已启用' : '已禁用' }}
        </button>

        <!-- 清空过滤器 -->
        <button
          @click="proxyStore.resetFilters"
          class="flex items-center gap-1 px-3 py-1 bg-red-500/20 border border-red-500/30 rounded-md text-sm text-red-300 hover:bg-red-500/30 transition-all duration-300"
        >
          <span class="text-xs">🧹</span>
          清空
        </button>
      </div>
    </div>

    <!-- 过滤条件表单 -->
    <div class="grid grid-cols-1 md:grid-cols-3 gap-3">
      <!-- 源IP -->
      <div class="space-y-1">
        <label class="block text-sm font-medium text-slate-300">源IP地址</label>
        <input
          type="text"
          :value="proxyStore.filters.srcIp"
          @input="updateFilterValue('srcIp', ($event.target as HTMLInputElement).value)"
          placeholder="如: 192.168.1"
          class="w-full px-3 py-1.5 bg-slate-900/60 border border-blue-500/30 rounded-md text-slate-200 text-sm focus:outline-none focus:border-blue-500/60 focus:ring-1 focus:ring-blue-500/20 placeholder-slate-500"
        />
      </div>

      <!-- 目标IP -->
      <div class="space-y-1">
        <label class="block text-sm font-medium text-slate-300">目标IP地址</label>
        <input
          type="text"
          :value="proxyStore.filters.dstIp"
          @input="updateFilterValue('dstIp', ($event.target as HTMLInputElement).value)"
          placeholder="如: 10.0.0"
          class="w-full px-3 py-1.5 bg-slate-900/60 border border-blue-500/30 rounded-md text-slate-200 text-sm focus:outline-none focus:border-blue-500/60 focus:ring-1 focus:ring-blue-500/20 placeholder-slate-500"
        />
      </div>

      <!-- URL路径 -->
      <div class="space-y-1">
        <label class="block text-sm font-medium text-slate-300">URL路径包含</label>
        <input
          type="text"
          :value="proxyStore.filters.url"
          @input="updateFilterValue('url', ($event.target as HTMLInputElement).value)"
          placeholder="如: api、.jpg、login"
          class="w-full px-3 py-1.5 bg-slate-900/60 border border-blue-500/30 rounded-md text-slate-200 text-sm focus:outline-none focus:border-blue-500/60 focus:ring-1 focus:ring-blue-500/20 placeholder-slate-500"
        />
      </div>
    </div>

    <!-- 过滤结果统计 -->
    <div v-if="proxyStore.filters.enabled" class="mt-3 pt-3 border-t border-blue-500/20">
      <div class="flex items-center gap-4 text-sm">
        <div class="flex items-center gap-2">
          <div class="w-2 h-2 bg-blue-400 rounded-full"></div>
          <span class="text-slate-300">显示:</span>
          <span class="text-blue-300 font-semibold">{{ proxyStore.filteredPacketCount }}</span>
          <span class="text-slate-400">/ {{ proxyStore.packetCount }}</span>
        </div>

        <div v-if="proxyStore.filteredPacketCount !== proxyStore.packetCount" class="text-slate-400">
          已过滤 {{ proxyStore.packetCount - proxyStore.filteredPacketCount }} 个数据包
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* 输入框聚焦时的额外效果 */
input:focus, select:focus {
  box-shadow: 0 0 0 1px rgba(59, 130, 246, 0.3);
}

/* 选项卡的悬停效果 */
select option {
  background-color: #1e293b;
  color: #e2e8f0;
}
</style>
