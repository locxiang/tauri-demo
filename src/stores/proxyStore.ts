import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { invoke, Channel } from '@tauri-apps/api/core';

// 定义数据包类型
export interface PacketData {
  id: string;
  timestamp: number;
  type: 'request' | 'response';
  protocol: string;
  srcIp: string;
  srcPort: number;
  dstIp: string;
  dstPort: number;
  length: number;
  http: {
    // 请求字段
    method?: string;
    path?: string;

    // 响应字段
    status_code?: number;
    status_text?: string;

    // 通用字段
    version: string;
    host: string;
    content_type: string;
    content_length?: number;
    headers: Record<string, string>;
    body: string;
  };
}

// 定义网络设备类型
export interface NetworkDevice {
  name: string;
  description: string;
  is_loopback: boolean;
  addresses: string[];
}

// 定义捕获状态类型
export interface CaptureStatus {
  running: boolean;
  message: string;
  device_name: string;
  start_time: number;
}

// 定义后端 HTTP 数据包类型（与Rust结构体对应）
export interface HttpPacket {
  id: number;
  timestamp: number;
  src_ip: string;
  src_port: number;
  dst_ip: string;
  dst_port: number;
  packet_type: string; // "request" 或 "response"

  // 请求字段
  method?: string;
  path?: string;

  // 响应字段
  status_code?: number;
  status_text?: string;

  // 通用字段
  version: string;
  host: string;
  content_type: string;
  content_length?: number;
  headers: [string, string][];
  body: string;
}

export const useProxyStore = defineStore('proxy', () => {
  // 状态
  const captureStatus = ref<CaptureStatus>({
    running: false,
    message: '未初始化',
    device_name: '',
    start_time: 0
  });

  const packets = ref<PacketData[]>([]);
  const devices = ref<NetworkDevice[]>([]);
  const selectedDevice = ref<string>('');
  const error = ref<string>('');
  const isLoading = ref<boolean>(false);

  // 过滤相关状态
  const filters = ref({
    enabled: false,
    type: '' as '' | 'request' | 'response',
    method: '' as string,
    statusCode: '' as string,
    srcIp: '' as string,
    dstIp: '' as string,
    port: '' as string,
    url: '' as string,
  });

  // 计算属性
  const isCapturing = computed(() => captureStatus.value.running);

  // 过滤后的数据包
  const filteredPackets = computed(() => {
    if (!filters.value.enabled) {
      return packets.value;
    }

    return packets.value.filter(packet => {
      // 类型过滤
      if (filters.value.type && packet.type !== filters.value.type) {
        return false;
      }

      // HTTP方法过滤
      if (filters.value.method && packet.http.method !== filters.value.method) {
        return false;
      }

      // 状态码过滤
      if (filters.value.statusCode) {
        const statusFilter = filters.value.statusCode;
        const statusCode = packet.http.status_code;

        if (statusFilter.includes('-')) {
          // 范围过滤 (如: 200-299)
          const [min, max] = statusFilter.split('-').map(Number);
          if (!statusCode || statusCode < min || statusCode > max) {
            return false;
          }
        } else {
          // 精确匹配
          if (statusCode !== Number(statusFilter)) {
            return false;
          }
        }
      }

      // 源IP过滤
      if (filters.value.srcIp && !packet.srcIp.includes(filters.value.srcIp)) {
        return false;
      }

      // 目标IP过滤
      if (filters.value.dstIp && !packet.dstIp.includes(filters.value.dstIp)) {
        return false;
      }

      // 端口过滤
      if (filters.value.port) {
        const port = Number(filters.value.port);
        if (packet.srcPort !== port && packet.dstPort !== port) {
          return false;
        }
      }

      // URL路径过滤
      if (filters.value.url) {
        const url = `${packet.http.host || ''}${packet.http.path || ''}`;
        if (!url.toLowerCase().includes(filters.value.url.toLowerCase())) {
          return false;
        }
      }

      return true;
    });
  });

  const packetCount = computed(() => packets.value.length);
  const filteredPacketCount = computed(() => filteredPackets.value.length);
  const requestCount = computed(() => packets.value.filter(p => p.type === 'request').length);
  const responseCount = computed(() => packets.value.filter(p => p.type === 'response').length);

  // 将后端HttpPacket转换为前端PacketData
  const convertHttpPacketToPacketData = (httpPacket: HttpPacket): PacketData => {
    return {
      id: httpPacket.id.toString(),
      timestamp: httpPacket.timestamp * 1000, // 转换为毫秒
      type: httpPacket.packet_type as 'request' | 'response',
      protocol: 'HTTP',
      srcIp: httpPacket.src_ip,
      srcPort: httpPacket.src_port,
      dstIp: httpPacket.dst_ip,
      dstPort: httpPacket.dst_port,
      length: httpPacket.body.length,
      http: {
        // 请求字段
        method: httpPacket.method,
        path: httpPacket.path,

        // 响应字段
        status_code: httpPacket.status_code,
        status_text: httpPacket.status_text,

        // 通用字段
        version: httpPacket.version,
        host: httpPacket.host,
        content_type: httpPacket.content_type,
        content_length: httpPacket.content_length,
        headers: Object.fromEntries(httpPacket.headers),
        body: httpPacket.body
      }
    };
  };

  // 初始化
  const initialize = async () => {
    try {
      isLoading.value = true;
      error.value = '';

      // 设置状态通道
      const statusChannel = new Channel<CaptureStatus>();
      statusChannel.onmessage = (status: CaptureStatus) => {
        console.log('收到状态更新:', status);
        captureStatus.value = status;
      };

      // 设置HTTP数据包通道
      const httpChannel = new Channel<HttpPacket>();
      httpChannel.onmessage = (httpPacket: HttpPacket) => {
        console.log(`收到 HTTP ${httpPacket.packet_type === 'request' ? '请求' : '响应'}:`, httpPacket);

        const packet = convertHttpPacketToPacketData(httpPacket);
        packets.value.unshift(packet);

        // 限制包数量，避免内存溢出
        if (packets.value.length > 1000) {
          packets.value = packets.value.slice(0, 1000);
        }
      };

      // 发送通道到后端
      await invoke('set_status_channel', { channel: statusChannel });
      await invoke('set_http_channel', { channel: httpChannel });

      // 获取初始状态
      await getCaptureStatus();

      // 获取网络设备列表
      await getNetworkDevices();

    } catch (err) {
      error.value = `初始化失败: ${err}`;
      console.error('初始化失败:', err);
    } finally {
      isLoading.value = false;
    }
  };

  // 开始捕获
  const startCapture = async () => {
    try {
      isLoading.value = true;
      error.value = '';

      // 如果用户没有选择设备，传递 null 让后端自动选择
      const deviceToUse = selectedDevice.value || null;
      await invoke('init_capture', { device_name: deviceToUse });

    } catch (err) {
      error.value = `启动失败: ${err}`;
      console.error('启动捕获失败:', err);
    } finally {
      isLoading.value = false;
    }
  };

  // 停止捕获
  const stopCapture = async () => {
    try {
      isLoading.value = true;
      error.value = '';

      await invoke('stop_capture');

    } catch (err) {
      error.value = `停止失败: ${err}`;
      console.error('停止捕获失败:', err);
    } finally {
      isLoading.value = false;
    }
  };

  // 清空数据包
  const clearPackets = () => {
    packets.value = [];
  };

  // 获取捕获状态
  const getCaptureStatus = async () => {
    try {
      const status = await invoke('get_capture_status') as CaptureStatus;
      captureStatus.value = status;
      return status;
    } catch (err) {
      console.error('获取状态失败:', err);
      throw err;
    }
  };

  // 检查权限
  const checkPermissions = async () => {
    try {
      const hasPermission = await invoke('has_pcap') as boolean;
      if (!hasPermission) {
        error.value = '缺少网络捕获权限，请安装 ChmodBPF 权限包';
      }
      return hasPermission;
    } catch (err) {
      console.error('权限检查失败:', err);
      throw err;
    }
  };

  // 获取网络设备列表
  const getNetworkDevices = async () => {
    try {
      isLoading.value = true;
      error.value = '';

      const deviceList = await invoke('get_network_devices') as NetworkDevice[];
      devices.value = deviceList;

      // 自动选择第一个非回环设备
      const nonLoopbackDevice = deviceList.find(device => !device.is_loopback);
      if (nonLoopbackDevice && !selectedDevice.value) {
        selectedDevice.value = nonLoopbackDevice.name;
      }

      return deviceList;
    } catch (err) {
      error.value = `获取网络设备失败: ${err}`;
      console.error('获取网络设备失败:', err);
      throw err;
    } finally {
      isLoading.value = false;
    }
  };

    // 清理资源
  const cleanup = async () => {
    try {
      if (captureStatus.value.running) {
        localStorage.clear();
        await stopCapture();
      }
    } catch (err) {
      console.error('清理失败:', err);
    }
  };

  // 保存数据包到 localStorage 用于新窗口访问
  const savePacketForWindow = (packet: PacketData) => {
    try {
      localStorage.setItem(`packet-${packet.id}`, JSON.stringify(packet));
    } catch (err) {
      console.error('保存数据包失败:', err);
    }
  };

  // 从 localStorage 获取数据包
  const getPacketForWindow = (id: string): PacketData | null => {
    try {
      const packetData = localStorage.getItem(`packet-${id}`);
      console.log('获取数据包 getPacketForWindow', id, packetData);
      if (packetData) {
        return JSON.parse(packetData) as PacketData;
      }
    } catch (err) {
      console.error('获取数据包失败:', err);
    }
    return null;
  };

  // 根据ID查找数据包
  const findPacketById = (id: string): PacketData | undefined => {
    return packets.value.find(packet => packet.id === id);
  };

  // 过滤相关方法
  const toggleFilter = () => {
    filters.value.enabled = !filters.value.enabled;
  };

  const resetFilters = () => {
    filters.value = {
      enabled: false,
      type: '',
      method: '',
      statusCode: '',
      srcIp: '',
      dstIp: '',
      port: '',
      url: '',
    };
  };

  const updateFilter = (key: keyof typeof filters.value, value: string | boolean) => {
    (filters.value as any)[key] = value;
  };

  return {
    // 状态
    captureStatus,
    packets,
    devices,
    selectedDevice,
    error,
    isLoading,
    filters,

    // 计算属性
    isCapturing,
    packetCount,
    filteredPacketCount,
    requestCount,
    responseCount,
    filteredPackets,

    // 方法
    initialize,
    startCapture,
    stopCapture,
    clearPackets,
    getCaptureStatus,
    checkPermissions,
    getNetworkDevices,
    cleanup,
    savePacketForWindow,
    getPacketForWindow,
    findPacketById,

    // 过滤方法
    toggleFilter,
    resetFilters,
    updateFilter
  };
});
