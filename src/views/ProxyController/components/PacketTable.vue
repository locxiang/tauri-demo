<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { useProxyStore } from '../../../stores/proxyStore';
import type { PacketData } from '../../../stores/proxyStore';

// 导入SVG图标
import HttpRequestIcon from '../../../assets/icons/http-request.svg';
import HttpResponseIcon from '../../../assets/icons/http-response.svg';
import NetworkPacketIcon from '../../../assets/icons/network-packet.svg';

defineProps<{
  packets: PacketData[]
}>();

const proxyStore = useProxyStore();

const formatTime = (timestamp: number): string => {
  const date = new Date(timestamp);
  return date.toLocaleTimeString('zh-CN', {
    hour12: false,
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit',
  });
};

const getPacketTypeClass = (type: string): string => {
  if (type === 'request') return 'border-l-cyan-500 bg-cyan-500/5';
  if (type === 'response') return 'border-l-green-500 bg-green-500/5';
  return 'border-l-gray-500 bg-gray-500/5';
};

const getPacketTypeIcon = (type: string): string => {
  if (type === 'request') return HttpRequestIcon;
  if (type === 'response') return HttpResponseIcon;
  return NetworkPacketIcon;
};

const getMethodClass = (method?: string): string => {
  if (!method) return 'bg-gray-500/20 text-gray-300';
  const methodClasses: Record<string, string> = {
    'GET': 'bg-blue-500/20 text-blue-300 border-blue-500/30',
    'POST': 'bg-green-500/20 text-green-300 border-green-500/30',
    'PUT': 'bg-yellow-500/20 text-yellow-300 border-yellow-500/30',
    'DELETE': 'bg-red-500/20 text-red-300 border-red-500/30',
    'PATCH': 'bg-purple-500/20 text-purple-300 border-purple-500/30',
    'OPTIONS': 'bg-orange-500/20 text-orange-300 border-orange-500/30',
    'HEAD': 'bg-gray-500/20 text-gray-300 border-gray-500/30'
  };
  return methodClasses[method] || 'bg-gray-500/20 text-gray-300 border-gray-500/30';
};

const getStatusCodeClass = (statusCode?: number): string => {
  if (!statusCode) return 'bg-gray-500/20 text-gray-300 border-gray-500/30';

  if (statusCode >= 200 && statusCode < 300) return 'bg-green-500/20 text-green-300 border-green-500/30';
  if (statusCode >= 300 && statusCode < 400) return 'bg-yellow-500/20 text-yellow-300 border-yellow-500/30';
  if (statusCode >= 400 && statusCode < 500) return 'bg-orange-500/20 text-orange-300 border-orange-500/30';
  if (statusCode >= 500) return 'bg-red-500/20 text-red-300 border-red-500/30';

  return 'bg-gray-500/20 text-gray-300 border-gray-500/30';
};

const formatPacketInfo = (packet: PacketData): string => {
  if (packet.type === 'request') {
    // 对于请求，显示 host + path
    const host = packet.http.host || '';
    const path = packet.http.path || '/';
    return `${host}${path}`;
  } else if (packet.type === 'response') {
    // 对于响应，显示状态信息
    const statusCode = packet.http.status_code || 0;
    const statusText = packet.http.status_text || 'Unknown';
    return `${statusCode} ${statusText}`;
  }
  return 'Unknown';
};

const openPacketDetail = async (packet: PacketData) => {
  try {
    console.log('openPacketDetail', packet);

    // 保存数据包
    proxyStore.savePacketForWindow(packet);

    // 使用 Tauri 命令创建新窗口
    await invoke('create_packet_window', {
      title: `${packet.type === 'request' ? '请求' : '响应'} - ${packet.id}`,
      url: `/packet-detail/${packet.id}`,
      width: 900,
      height: 700
    });

    // 记录已打开的窗口
    console.log('Tauri 新窗口创建成功');

  } catch (err) {
    console.error('打开数据包详情窗口失败:', err);
    // 如果 Tauri 窗口创建失败，显示错误提示
    alert(`打开窗口失败: ${err}`);
  }
};
</script>

<template>
  <div class="bg-gradient-to-br from-slate-900/60 to-slate-800/40 backdrop-blur-lg border border-blue-500/10 rounded-lg overflow-hidden">
    <!-- 表头 -->
    <div class="grid grid-cols-[80px_120px_140px_120px_120px_100px_1fr_120px] gap-2 bg-gradient-to-r from-slate-900/80 to-slate-800/60 backdrop-blur-lg border-b border-blue-500/20 px-4 py-3">
      <div class="text-xs font-semibold text-slate-300 uppercase tracking-wide">序号</div>
      <div class="text-xs font-semibold text-slate-300 uppercase tracking-wide">时间</div>
      <div class="text-xs font-semibold text-slate-300 uppercase tracking-wide">类型</div>
      <div class="text-xs font-semibold text-slate-300 uppercase tracking-wide">源地址</div>
      <div class="text-xs font-semibold text-slate-300 uppercase tracking-wide">目标地址</div>
      <div class="text-xs font-semibold text-slate-300 uppercase tracking-wide">长度</div>
      <div class="text-xs font-semibold text-slate-300 uppercase tracking-wide">信息</div>
      <div class="text-xs font-semibold text-slate-300 uppercase tracking-wide">操作</div>
    </div>

    <!-- 表体 -->
    <div class="max-h-[310px] overflow-y-auto">
      <!-- 空状态 -->
      <div v-if="packets.length === 0" class="flex flex-col items-center justify-center py-16 text-center">
        <img :src="NetworkPacketIcon" alt="网络数据包" class="w-16 h-16 mb-4 opacity-50 text-slate-400" />
        <div class="text-slate-400 text-lg font-medium mb-2">暂无数据包</div>
        <div class="text-slate-500 text-sm">请选择网络设备并开始抓包</div>
      </div>

      <!-- 数据包列表 -->
      <template v-else>
        <div
          v-for="(packet, index) in packets"
          :key="packet.id"
          :class="['grid grid-cols-[80px_120px_140px_120px_120px_100px_1fr_120px] gap-2 px-4 py-3 border-b border-blue-500/5 hover:bg-blue-500/10 transition-all duration-200 cursor-pointer border-l-4', getPacketTypeClass(packet.type)]"
          @click="openPacketDetail(packet)"
        >
          <!-- 序号 -->
          <div class="text-sm font-mono text-slate-400 flex items-center">
            {{ packets.length - index }}
          </div>

          <!-- 时间 -->
          <div class="text-sm font-mono text-slate-400 flex items-center">
            {{ formatTime(packet.timestamp) }}
          </div>

          <!-- 类型 -->
          <div class="flex items-center gap-2">
            <img :src="getPacketTypeIcon(packet.type)"
                 :alt="packet.type === 'request' ? 'HTTP请求' : 'HTTP响应'"
                 :class="['w-4 h-4', packet.type === 'request' ? 'text-cyan-400' : 'text-green-400']" />
            <div class="flex flex-col gap-1">
              <!-- 请求类型：显示方法 -->
              <span v-if="packet.type === 'request' && packet.http.method"
                    :class="['inline-flex items-center px-2 py-0.5 rounded text-xs font-semibold uppercase tracking-wide border transition-all duration-200', getMethodClass(packet.http.method)]">
                {{ packet.http.method }}
              </span>
              <!-- 响应类型：显示状态码 -->
              <span v-else-if="packet.type === 'response' && packet.http.status_code"
                    :class="['inline-flex items-center px-2 py-0.5 rounded text-xs font-semibold tracking-wide border transition-all duration-200', getStatusCodeClass(packet.http.status_code)]">
                {{ packet.http.status_code }}
              </span>
              <!-- 类型标签 -->
              <span :class="['text-xs uppercase tracking-wide font-medium',
                           packet.type === 'request' ? 'text-cyan-400' : 'text-green-400']">
              </span>
            </div>
          </div>

          <!-- 源地址 -->
          <div class="text-sm font-mono text-slate-300 flex items-center">
            <div class="truncate">
              <div class="text-xs text-slate-400">{{ packet.srcIp }}</div>
              <div class="text-xs text-slate-500">:{{ packet.srcPort }}</div>
            </div>
          </div>

          <!-- 目标地址 -->
          <div class="text-sm font-mono text-slate-300 flex items-center">
            <div class="truncate">
              <div class="text-xs text-slate-400">{{ packet.dstIp }}</div>
              <div class="text-xs text-slate-500">:{{ packet.dstPort }}</div>
            </div>
          </div>

          <!-- 长度 -->
          <div class="text-sm font-mono text-slate-400 flex items-center">
            {{ packet.length }} B
          </div>

          <!-- 信息 -->
          <div class="text-sm text-slate-300 flex items-center truncate">
            <span class="truncate">{{ formatPacketInfo(packet) }}</span>
          </div>

          <!-- 操作 -->
          <div class="flex items-center">
            <button
              @click.stop="openPacketDetail(packet)"
              class="flex items-center gap-1 px-3 py-1 bg-gradient-to-r from-blue-500/10 to-cyan-500/10 border border-blue-500/30 rounded-md text-xs text-slate-200 hover:from-blue-500/20 hover:to-cyan-500/20 hover:border-blue-500/50 hover:-translate-y-0.5 transition-all duration-300 relative overflow-hidden group"
            >
              <span class="text-xs">👁️</span>
              <span class="font-medium">详情</span>
              <div class="absolute inset-0 bg-gradient-to-r from-white/0 via-white/5 to-white/0 -translate-x-full group-hover:translate-x-full transition-transform duration-500"></div>
            </button>
          </div>
        </div>
      </template>
    </div>
  </div>
</template>

<style scoped>
/* 自定义滚动条样式 */
::-webkit-scrollbar {
  width: 8px;
}

::-webkit-scrollbar-track {
  background: rgba(30, 41, 59, 0.5);
  border-radius: 4px;
}

::-webkit-scrollbar-thumb {
  background: linear-gradient(135deg, rgba(59, 130, 246, 0.6), rgba(6, 182, 212, 0.6));
  border-radius: 4px;
}

::-webkit-scrollbar-thumb:hover {
  background: linear-gradient(135deg, rgba(59, 130, 246, 0.8), rgba(6, 182, 212, 0.8));
}

/* SVG图标样式 */
img[alt="HTTP请求"], img[alt="HTTP响应"], img[alt="网络数据包"] {
  filter: brightness(0) saturate(100%) invert(71%) sepia(11%) saturate(1398%) hue-rotate(165deg) brightness(91%) contrast(85%);
}

img[alt="HTTP请求"] {
  filter: brightness(0) saturate(100%) invert(62%) sepia(98%) saturate(1946%) hue-rotate(158deg) brightness(103%) contrast(88%);
}

img[alt="HTTP响应"] {
  filter: brightness(0) saturate(100%) invert(48%) sepia(98%) saturate(1295%) hue-rotate(85deg) brightness(98%) contrast(97%);
}
</style>
