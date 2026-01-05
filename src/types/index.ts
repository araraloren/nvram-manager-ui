// 状态级别类型
export type StatusLevel = 'info' | 'warning' | 'error';

// 状态更新类型
export interface StatusUpdate {
  level: StatusLevel;
  message: string;
}

// PS版本类型
export interface PsVersion {
  version: string;
  build: string;
  path: string;
}

// JChip版本类型
export interface JChipVersion {
  version: string;
  build: string;
  path: string;
}

// NVRAM备份类型
export interface NvramBackup {
  id: number;
  nvramName: string;
  psVersion: PsVersion;
  jchipVersion: JChipVersion;
  backupTime: string;
  fileList: string[];
  gamePath: string;
}

// 带NVRAM路径的信息类型（用于事件监听）
export interface NvramInfoWithPath extends NvramBackup {
  nvramPath: string;
}

// App配置类型
export interface AppConfig {
  nvramPath: string;
  backupPath: string;
  clearAfterBackup: boolean;
  clearNvramOnRestore: boolean;
  clearBackupOnRestore: boolean;
}
