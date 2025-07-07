<script setup lang="ts">
import { ref, computed, watch, onMounted } from 'vue';
import type { PacketData } from '../../../stores/proxyStore';

const props = defineProps<{
  packet: PacketData
}>();

const activeTab = ref<'overview' | 'http' | 'headers' | 'body'>('overview');

// 添加调试信息
onMounted(() => {
  console.log('PacketDetail 组件挂载，接收到的数据:', props.packet);
});

// 监听 activeTab 变化并打印日志
watch(activeTab, (newTab, oldTab) => {
  console.log(`标签页切换: 从 ${oldTab} 切换到 ${newTab}`);
});

// 监听 packet 变化
watch(() => props.packet, (newPacket) => {
  console.log('数据包数据发生变化:', newPacket);
});

const formatTime = (timestamp: number): string => {
  try {
    if (!timestamp || typeof timestamp !== 'number') {
      return '无效时间';
    }
    const date = new Date(timestamp);
    return date.toLocaleString('zh-CN', {
      year: 'numeric',
      month: '2-digit',
      day: '2-digit',
      hour: '2-digit',
      minute: '2-digit',
      second: '2-digit',
      hour12: false
    });
  } catch (err) {
    console.error('时间格式化错误:', err);
    return '时间格式错误';
  }
};

const hasHttpDetails = computed(() => {
  try {
    if (!props.packet?.http) return false;
    return !!(
      props.packet.http.method ||
      props.packet.http.path ||
      props.packet.http.host ||
      props.packet.http.status_code ||
      props.packet.http.status_text
    );
  } catch (err) {
    console.error('计算 hasHttpDetails 错误:', err);
    return false;
  }
});

const hasHeaders = computed(() => {
  try {
    if (!props.packet?.http?.headers) return false;
    return Object.keys(props.packet.http.headers).length > 0;
  } catch (err) {
    console.error('计算 hasHeaders 错误:', err);
    return false;
  }
});

const hasBody = computed(() => {
  try {
    if (!props.packet?.http?.body) return false;
    return props.packet.http.body.trim().length > 0;
  } catch (err) {
    console.error('计算 hasBody 错误:', err);
    return false;
  }
});

const isRequest = computed(() => props.packet?.type === 'request');
const isResponse = computed(() => props.packet?.type === 'response');

const copyToClipboard = async (text: string) => {
  try {
    await navigator.clipboard.writeText(text);
    console.log('复制成功');
  } catch (err) {
    console.error('复制失败:', err);
  }
};

const getMethodColor = (method: string): string => {
  if (!method) return 'text-gray-400';
  const colors: Record<string, string> = {
    GET: 'text-green-400',
    POST: 'text-blue-400',
    PUT: 'text-yellow-400',
    DELETE: 'text-red-400',
    PATCH: 'text-purple-400'
  };
  return colors[method] || 'text-gray-400';
};

const getTypeColor = (type: string): string => {
  if (!type) return 'text-gray-400';
  if (type === 'request') return 'text-cyan-400';
  if (type === 'response') return 'text-green-400';
  return 'text-gray-400';
};

const getStatusCodeColor = (statusCode?: number): string => {
  if (!statusCode) return 'text-gray-400';

  if (statusCode >= 200 && statusCode < 300) return 'text-green-400';
  if (statusCode >= 300 && statusCode < 400) return 'text-yellow-400';
  if (statusCode >= 400 && statusCode < 500) return 'text-orange-400';
  if (statusCode >= 500) return 'text-red-400';

  return 'text-gray-400';
};

const formatFullUrl = computed(() => {
  if (isRequest.value && props.packet.http.host && props.packet.http.path) {
    return `${props.packet.http.host}${props.packet.http.path}`;
  }
  return '';
});
</script>

<template>
  <div class="h-screen bg-gray-900 text-white overflow-hidden">
    <!-- 检查数据是否存在 -->
    <div v-if="!packet" class="flex items-center justify-center h-full">
      <div class="text-center">
        <div class="text-4xl mb-4">⚠️</div>
        <div class="text-xl">数据包数据为空</div>
        <div class="text-sm text-gray-400 mt-2">请检查路由参数和数据加载</div>
      </div>
    </div>

    <!-- 主要内容 -->
    <div v-else class="flex flex-col h-full">
      <!-- 简化的头部 -->
      <div class="bg-gray-800 border-b border-gray-700 p-4">
        <div class="flex items-center justify-between">
          <div>
            <h1 class="text-xl font-semibold">
              {{ isRequest ? 'HTTP 请求' : 'HTTP 响应' }} #{{ packet.id || '未知' }}
            </h1>
            <p class="text-sm text-gray-400">{{ formatTime(packet.timestamp) }}</p>
          </div>
          <div class="flex items-center space-x-3">
            <span :class="['px-3 py-1 text-sm rounded', getTypeColor(packet.type)]">
              {{ isRequest ? '📤 请求' : '📥 响应' }}
            </span>
            <!-- 显示方法（请求）或状态码（响应） -->
            <span v-if="isRequest && packet.http.method"
                  :class="['px-3 py-1 text-sm font-bold rounded bg-gray-700', getMethodColor(packet.http.method)]">
              {{ packet.http.method }}
            </span>
            <span v-else-if="isResponse && packet.http.status_code"
                  :class="['px-3 py-1 text-sm font-bold rounded bg-gray-700', getStatusCodeColor(packet.http.status_code)]">
              {{ packet.http.status_code }} {{ packet.http.status_text }}
            </span>
          </div>
        </div>
      </div>

      <!-- 简化的选项卡 -->
      <div class="bg-gray-800 border-b border-gray-700">
        <div class="flex px-4">
          <button
            @click="activeTab = 'overview'"
            :class="[
              'px-4 py-3 text-sm border-b-2 transition-colors',
              activeTab === 'overview'
                ? 'border-blue-500 text-blue-400'
                : 'border-transparent text-gray-400 hover:text-white'
            ]"
          >
            概览
          </button>
          <button
            v-if="hasHttpDetails"
            @click="activeTab = 'http'"
            :class="[
              'px-4 py-3 text-sm border-b-2 transition-colors',
              activeTab === 'http'
                ? 'border-blue-500 text-blue-400'
                : 'border-transparent text-gray-400 hover:text-white'
            ]"
          >
            HTTP 详情
          </button>
          <button
            v-if="hasHeaders"
            @click="activeTab = 'headers'"
            :class="[
              'px-4 py-3 text-sm border-b-2 transition-colors',
              activeTab === 'headers'
                ? 'border-blue-500 text-blue-400'
                : 'border-transparent text-gray-400 hover:text-white'
            ]"
          >
            {{ isRequest ? '请求头' : '响应头' }}
          </button>
          <button
            v-if="hasBody"
            @click="activeTab = 'body'"
            :class="[
              'px-4 py-3 text-sm border-b-2 transition-colors',
              activeTab === 'body'
                ? 'border-blue-500 text-blue-400'
                : 'border-transparent text-gray-400 hover:text-white'
            ]"
          >
            {{ isRequest ? '请求体' : '响应体' }}
          </button>
        </div>
      </div>

      <!-- 内容区域 -->
      <div class="flex-1 overflow-y-auto p-6">
        <!-- 概览 -->
        <div v-if="activeTab === 'overview'">
          <div class="space-y-4">
             <!-- HTTP 信息 -->
             <div v-if="hasHttpDetails" class="bg-gray-800 rounded p-4">
              <h3 class="text-lg font-semibold mb-3 text-yellow-400">HTTP 信息</h3>
              <div class="space-y-2">
                <!-- 请求特定信息 -->
                <template v-if="isRequest">
                  <div v-if="packet.http.method" class="flex justify-between">
                    <span class="text-gray-400">请求方法:</span>
                  <span :class="['font-bold', getMethodColor(packet.http.method)]">{{ packet.http.method }}</span>
                </div>
                  <div v-if="packet.http.path" class="flex justify-between">
                    <span class="text-gray-400">请求路径:</span>
                    <span class="font-mono text-sm">{{ packet.http.path }}</span>
                  </div>
                </template>
                <!-- 响应特定信息 -->
                <template v-else-if="isResponse">
                  <div v-if="packet.http.status_code" class="flex justify-between">
                    <span class="text-gray-400">状态码:</span>
                    <span :class="['font-bold', getStatusCodeColor(packet.http.status_code)]">
                      {{ packet.http.status_code }}
                    </span>
                  </div>
                  <div v-if="packet.http.status_text" class="flex justify-between">
                    <span class="text-gray-400">状态文本:</span>
                    <span class="font-mono text-sm">{{ packet.http.status_text }}</span>
                  </div>
                </template>
                <!-- 通用信息 -->
                <div v-if="packet.http.host" class="flex justify-between">
                  <span class="text-gray-400">主机:</span>
                  <span class="font-mono text-sm">{{ packet.http.host }}</span>
                </div>
                <div v-if="packet.http.version" class="flex justify-between">
                  <span class="text-gray-400">HTTP版本:</span>
                  <span class="font-mono text-sm">{{ packet.http.version }}</span>
                </div>
                <div v-if="packet.http.content_type" class="flex justify-between">
                  <span class="text-gray-400">内容类型:</span>
                  <span class="font-mono text-sm">{{ packet.http.content_type }}</span>
                </div>
                <div v-if="packet.http.content_length" class="flex justify-between">
                  <span class="text-gray-400">内容长度:</span>
                  <span class="font-mono text-sm">{{ packet.http.content_length }} 字节</span>
                </div>
                <!-- 完整URL（仅请求） -->
                <div v-if="formatFullUrl">
                  <div class="flex justify-between items-center mb-2">
                    <span class="text-gray-400">完整URL:</span>
                    <button
                      @click="copyToClipboard(formatFullUrl)"
                      class="px-2 py-1 text-xs bg-blue-600 hover:bg-blue-700 rounded"
                    >
                      复制
                    </button>
                  </div>
                  <div class="bg-gray-900 p-2 rounded font-mono text-sm break-all">
                    {{ formatFullUrl }}
                  </div>
                </div>
              </div>
            </div>
            <!-- 基本信息 -->
            <div class="bg-gray-800 rounded p-4">
              <h3 class="text-lg font-semibold mb-3 text-blue-400">基本信息</h3>
              <div class="space-y-2">
                <div class="flex justify-between">
                  <span class="text-gray-400">ID:</span>
                  <span>{{ packet.id || '未知' }}</span>
                </div>
                <div class="flex justify-between">
                  <span class="text-gray-400">时间:</span>
                  <span>{{ formatTime(packet.timestamp) }}</span>
                </div>
                <div class="flex justify-between">
                  <span class="text-gray-400">类型:</span>
                  <span :class="getTypeColor(packet.type)">
                    {{ isRequest ? '📤 HTTP 请求' : '📥 HTTP 响应' }}
                  </span>
                </div>
                <div class="flex justify-between">
                  <span class="text-gray-400">协议:</span>
                  <span>{{ packet.protocol || '未知' }}</span>
                </div>
                <div class="flex justify-between">
                  <span class="text-gray-400">大小:</span>
                  <span>{{ packet.length || 0 }} 字节</span>
                </div>
              </div>
            </div>

            <!-- 网络信息 -->
            <div class="bg-gray-800 rounded p-4">
              <h3 class="text-lg font-semibold mb-3 text-green-400">网络信息</h3>
              <div class="space-y-2">
                <div class="flex justify-between">
                  <span class="text-gray-400">源地址:</span>
                  <span class="font-mono text-sm">{{ packet.srcIp || '未知' }}:{{ packet.srcPort || 0 }}</span>
                </div>
                <div class="flex justify-between">
                  <span class="text-gray-400">目标地址:</span>
                  <span class="font-mono text-sm">{{ packet.dstIp || '未知' }}:{{ packet.dstPort || 0 }}</span>
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- HTTP 详情 -->
        <div v-if="activeTab === 'http' && hasHttpDetails">
          <div class="bg-gray-800 rounded p-4">
            <h3 class="text-lg font-semibold mb-3 text-blue-400">HTTP 详情</h3>
            <div class="space-y-3">
              <!-- 请求特定字段 -->
              <template v-if="isRequest">
                <div v-if="packet.http.method">
                <label class="block text-sm text-gray-400 mb-1">请求方法</label>
                <span :class="['inline-block px-2 py-1 rounded', getMethodColor(packet.http.method)]">
                  {{ packet.http.method }}
                </span>
              </div>
                <div v-if="packet.http.path">
                <label class="block text-sm text-gray-400 mb-1">请求路径</label>
                <div class="bg-gray-900 p-2 rounded font-mono text-sm">
                    {{ packet.http.path }}
                  </div>
                </div>
              </template>
              <!-- 响应特定字段 -->
              <template v-else-if="isResponse">
                <div v-if="packet.http.status_code">
                  <label class="block text-sm text-gray-400 mb-1">状态码</label>
                  <span :class="['inline-block px-2 py-1 rounded', getStatusCodeColor(packet.http.status_code)]">
                    {{ packet.http.status_code }}
                  </span>
                </div>
                <div v-if="packet.http.status_text">
                  <label class="block text-sm text-gray-400 mb-1">状态文本</label>
                  <div class="bg-gray-900 p-2 rounded font-mono text-sm">
                    {{ packet.http.status_text }}
                </div>
              </div>
              </template>
              <!-- 通用字段 -->
              <div v-if="packet.http.host">
                <label class="block text-sm text-gray-400 mb-1">主机地址</label>
                <div class="bg-gray-900 p-2 rounded font-mono text-sm">
                  {{ packet.http.host }}
                </div>
              </div>
              <div v-if="packet.http.version">
                <label class="block text-sm text-gray-400 mb-1">HTTP版本</label>
                <div class="bg-gray-900 p-2 rounded font-mono text-sm">
                  {{ packet.http.version }}
                </div>
              </div>
              <div v-if="packet.http.content_type">
                <label class="block text-sm text-gray-400 mb-1">内容类型</label>
                <div class="bg-gray-900 p-2 rounded font-mono text-sm">
                  {{ packet.http.content_type }}
                </div>
              </div>
              <div v-if="packet.http.content_length">
                <label class="block text-sm text-gray-400 mb-1">内容长度</label>
                <div class="bg-gray-900 p-2 rounded font-mono text-sm">
                  {{ packet.http.content_length }} 字节
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- 请求头/响应头 -->
        <div v-if="activeTab === 'headers' && hasHeaders">
          <div class="bg-gray-800 rounded p-4">
            <div class="flex justify-between items-center mb-3">
              <h3 class="text-lg font-semibold text-blue-400">
                {{ isRequest ? 'HTTP 请求头' : 'HTTP 响应头' }}
              </h3>
              <button
                @click="copyToClipboard(JSON.stringify(packet.http.headers, null, 2))"
                class="px-2 py-1 text-sm bg-blue-600 hover:bg-blue-700 rounded"
              >
                复制全部
              </button>
            </div>
            <div class="space-y-2">
              <div
                v-for="[key, value] in Object.entries(packet.http.headers || {})"
                :key="key"
                class="flex flex-col sm:flex-row sm:justify-between border-b border-gray-700 pb-2"
              >
                <div class="font-semibold text-blue-400 font-mono text-sm">{{ key }}</div>
                <div class="font-mono text-sm break-all">{{ value }}</div>
              </div>
            </div>
          </div>
        </div>

        <!-- 请求体/响应体 -->
        <div v-if="activeTab === 'body' && hasBody">
          <div class="bg-gray-800 rounded p-4">
            <div class="flex justify-between items-center mb-3">
              <h3 class="text-lg font-semibold text-blue-400">
                {{ isRequest ? 'HTTP 请求体' : 'HTTP 响应体' }}
              </h3>
              <button
                @click="copyToClipboard(packet.http.body || '')"
                class="px-2 py-1 text-sm bg-blue-600 hover:bg-blue-700 rounded"
              >
                复制
              </button>
            </div>
            <pre class="bg-gray-900 p-3 rounded font-mono text-sm whitespace-pre-wrap break-all max-h-80 overflow-y-auto">{{ packet.http.body }}</pre>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* 基础滚动条样式 */
::-webkit-scrollbar {
  width: 6px;
}

::-webkit-scrollbar-track {
  background: #374151;
}

::-webkit-scrollbar-thumb {
  background: #6b7280;
  border-radius: 3px;
}
</style>
