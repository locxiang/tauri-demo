import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { invoke, Channel } from '@tauri-apps/api/core';

// Token状态枚举
export type TokenState =
  | 'Waiting'           // 等待获取
  | 'Active'            // 已获取，有效
  | 'Expired'           // 已过期
  | { Failed: string }; // 获取失败

// Token状态接口
export interface TokenStatus {
  system_id: string;
  system_name: string;
  has_token: boolean;
  token_acquired_at?: number;
  token_expires_at?: number;
  last_seen_url?: string;
  status: TokenState;
}

// Token事件类型 - 匹配 Rust enum 的 serde 序列化格式
export type TokenEvent =
  | {
      type: 'TokenAcquired';
      system_id: string;
      system_name: string;
      token: string;
      acquired_at: number;
      expires_at: number;
      source_url: string;
    }
  | {
      type: 'TokenExpired';
      system_id: string;
      system_name: string;
      expired_at: number;
    }
  | {
      type: 'TokenFailed';
      system_id: string;
      system_name: string;
      error: string;
      failed_at: number;
    };

export const useAuthStore = defineStore('auth', () => {
  // 状态
  const tokenStatuses = ref<TokenStatus[]>([]);
  const tokenEvents = ref<TokenEvent[]>([]);
  const error = ref<string>('');
  const isLoading = ref<boolean>(false);

  const currentTime = ref<number>(0);

  // 计算属性
  const activeTokensCount = computed(() =>
    tokenStatuses.value.filter(status => status.status === 'Active').length
  );

  const expiredTokensCount = computed(() =>
    tokenStatuses.value.filter(status => status.status === 'Expired').length
  );

  const waitingTokensCount = computed(() =>
    tokenStatuses.value.filter(status => status.status === 'Waiting').length
  );

  const failedTokensCount = computed(() =>
    tokenStatuses.value.filter(status =>
      typeof status.status === 'object' && 'Failed' in status.status
    ).length
  );

  // 获取特定系统的状态
  const getSystemStatus = computed(() => (systemId: string) =>
    tokenStatuses.value.find(status => status.system_id === systemId)
  );

  // 检查系统是否有有效token
  const hasValidToken = computed(() => (systemId: string) => {
    const status = getSystemStatus.value(systemId);
    return status?.status === 'Active';
  });

  // 初始化
  const initialize = async () => {
    try {
      isLoading.value = true;
      error.value = '';

      console.log('🔐 开始初始化Auth Store...');

      // 创建定时器，每秒更新一次当前时间
      setInterval(() => {
        currentTime.value = Math.floor(Date.now() / 1000);
      }, 1000);


      // 设置Token事件通道
      const tokenEventChannel = new Channel<TokenEvent>();
      tokenEventChannel.onmessage = (event: TokenEvent) => {
        console.log('收到Token事件:', event);
        tokenEvents.value.unshift(event);

        // 限制事件历史数量（前端管理）
        if (tokenEvents.value.length > 100) {
          tokenEvents.value = tokenEvents.value.slice(0, 100);
        }

        // 直接根据事件更新对应系统的状态
        updateTokenStatusFromEvent(event);
      };

      // 发送通道到后端
      await invoke('set_token_event_channel', { channel: tokenEventChannel });
      console.log('✅ Token事件通道设置成功');

      // 获取初始状态
      await refreshTokenStatuses();

      console.log('🎉 Auth Store初始化完成');

    } catch (err) {
      error.value = `Auth Store初始化失败: ${err}`;
      console.error('Auth Store初始化失败:', err);
      throw err; // 重新抛出错误，让调用者知道初始化失败
    } finally {
      isLoading.value = false;
    }
  };

  // 根据事件直接更新对应系统的状态（避免重新获取所有状态）
  const updateTokenStatusFromEvent = (event: TokenEvent) => {
    const systemId = event.system_id;
    const systemName = event.system_name;

    // 查找现有状态
    const existingIndex = tokenStatuses.value.findIndex(status => status.system_id === systemId);

    if (existingIndex !== -1) {
      // 更新现有状态
      const existingStatus = tokenStatuses.value[existingIndex];

      if (event.type === 'TokenAcquired') {
        tokenStatuses.value[existingIndex] = {
          ...existingStatus,
          has_token: true,
          token_acquired_at: event.acquired_at,
          token_expires_at: event.expires_at,
          last_seen_url: event.source_url,
          status: 'Active' as TokenState,
        };
        console.log(`🎉 系统 [${systemName}] Token状态更新为有效`);
      } else if (event.type === 'TokenExpired') {
        tokenStatuses.value[existingIndex] = {
          ...existingStatus,
          status: 'Expired' as TokenState,
        };
        console.log(`⏰ 系统 [${systemName}] Token状态更新为过期`);
      } else if (event.type === 'TokenFailed') {
        tokenStatuses.value[existingIndex] = {
          ...existingStatus,
          status: { Failed: event.error } as TokenState,
        };
        console.log(`❌ 系统 [${systemName}] Token状态更新为失败: ${event.error}`);
      }
    } else {
      // 如果找不到现有状态，创建新的状态项
      let newStatus: TokenStatus;

      if (event.type === 'TokenAcquired') {
        newStatus = {
          system_id: systemId,
          system_name: systemName,
          has_token: true,
          token_acquired_at: event.acquired_at,
          token_expires_at: event.expires_at,
          last_seen_url: event.source_url,
          status: 'Active' as TokenState,
        };
      } else {
        newStatus = {
          system_id: systemId,
          system_name: systemName,
          has_token: false,
          status: event.type === 'TokenExpired' ? 'Expired' as TokenState :
                  (event.type === 'TokenFailed' ? { Failed: event.error } as TokenState : 'Waiting' as TokenState),
        };
      }

      tokenStatuses.value.push(newStatus);
      console.log(`📝 为系统 [${systemName}] 创建新的Token状态`);
    }
  };

  // 刷新所有系统的token状态（仅在初始化时调用）
  const refreshTokenStatuses = async () => {
    try {
      console.log('🔄 刷新所有系统token状态...');
      const statuses = await invoke('get_all_token_status') as TokenStatus[];
      tokenStatuses.value = statuses;
      console.log(`📊 获取到 ${statuses.length} 个系统状态`);
      return statuses; // 返回获取到的状态数据
    } catch (err) {
      console.error('刷新token状态失败:', err);
      throw err;
    }
  };

  // 获取特定系统的token
  const getSystemToken = async (systemId: string): Promise<string | null> => {
    try {
      console.log(`🔍 获取系统 [${systemId}] 的token...`);
      const token = await invoke('get_system_token', { systemId }) as string | null;

      if (token) {
        console.log(`✅ 系统 [${systemId}] token可用，长度: ${token.length}`);
      } else {
        console.log(`❌ 系统 [${systemId}] token不可用`);
      }

      return token;
    } catch (err) {
      console.error(`获取系统 [${systemId}] token失败:`, err);
      throw err;
    }
  };

  // 清除特定系统的token
  const clearSystemToken = async (systemId: string) => {
    try {
      isLoading.value = true;
      console.log(`🗑️ 清除系统 [${systemId}] 的token...`);

      await invoke('clear_system_token', { systemId });
      console.log(`✅ 系统 [${systemId}] token已清除`);

      // 直接更新本地状态，不需要全量刷新
      const existingIndex = tokenStatuses.value.findIndex(status => status.system_id === systemId);
      if (existingIndex !== -1) {
        tokenStatuses.value[existingIndex] = {
          ...tokenStatuses.value[existingIndex],
          has_token: false,
          token_acquired_at: undefined,
          token_expires_at: undefined,
          last_seen_url: undefined,
          status: 'Waiting' as TokenState,
        };
        console.log(`🔄 已更新系统 [${systemId}] 本地状态为等待状态`);
      }

    } catch (err) {
      error.value = `清除系统 [${systemId}] token失败: ${err}`;
      console.error(`清除系统 [${systemId}] token失败:`, err);
      throw err;
    } finally {
      isLoading.value = false;
    }
  };

  // 清除所有系统的token
  const clearAllTokens = async () => {
    try {
      isLoading.value = true;
      console.log('🗑️ 清除所有系统token...');

      await invoke('clear_all_tokens');
      console.log('✅ 所有系统token已清除');

      // 直接更新本地状态，不需要全量刷新
      tokenStatuses.value = tokenStatuses.value.map(status => ({
        ...status,
        has_token: false,
        token_acquired_at: undefined,
        token_expires_at: undefined,
        last_seen_url: undefined,
        status: 'Waiting' as TokenState,
      }));
      console.log('🔄 已更新所有系统本地状态为等待状态');

    } catch (err) {
      error.value = `清除所有token失败: ${err}`;
      console.error('清除所有token失败:', err);
      throw err;
    } finally {
      isLoading.value = false;
    }
  };

  // 清空错误信息
  const clearError = () => {
    error.value = '';
  };

  // 格式化token状态显示
  const formatTokenStatus = (status: TokenState): string => {
    if (status === 'Active') return '有效';
    if (status === 'Expired') return '已过期';
    if (status === 'Waiting') return '等待获取';
    if (typeof status === 'object' && 'Failed' in status) {
      return `失败: ${status.Failed}`;
    }
    return '未知';
  };

  // 格式化token事件显示
  const formatTokenEvent = (event: TokenEvent): string => {
    if (event.type === 'TokenAcquired') {
      return `[${event.system_name}] Token获取成功`;
    }
    if (event.type === 'TokenExpired') {
      return `[${event.system_name}] Token已过期`;
    }
    if (event.type === 'TokenFailed') {
      return `[${event.system_name}] Token获取失败: ${event.error}`;
    }
    return '未知事件';
  };

  // 获取token剩余时间（秒）
  const getTokenRemainingTime = computed(() => (systemId: string): number | null => {
    const status = getSystemStatus.value(systemId);
    if (!status || !status.token_expires_at || status.status !== 'Active') {
      return null;
    }


    const remaining = status.token_expires_at - currentTime.value;
    return remaining > 0 ? remaining : 0;
  });

  // 获取token剩余时间（格式化后）
  const getTokenRemainingTimeFormatted = computed(() => (systemId: string): string => {
    const remaining = getTokenRemainingTime.value(systemId);
    return remaining !== null ? formatRemainingTime(remaining) : '-';
  });

  // 格式化剩余时间显示
  const formatRemainingTime = (seconds: number): string => {
    if (seconds <= 0) return '已过期';

    const hours = Math.floor(seconds / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);
    const secs = seconds % 60;

    if (hours > 0) {
      return `${hours}小时${minutes}分钟`;
    } else if (minutes > 0) {
      return `${minutes}分${secs}秒`;
    } else {
      return `${secs}秒`;
    }
  };

  return {
    // 状态
    tokenStatuses,
    tokenEvents,
    error,
    isLoading,

    // 计算属性
    activeTokensCount,
    expiredTokensCount,
    waitingTokensCount,
    failedTokensCount,
    getSystemStatus,
    hasValidToken,
    getTokenRemainingTime,
    getTokenRemainingTimeFormatted,

    // 方法
    initialize,
    refreshTokenStatuses,
    getSystemToken,
    clearSystemToken,
    clearAllTokens,
    clearError,
    formatTokenStatus,
    formatTokenEvent,
    formatRemainingTime,
  };
});