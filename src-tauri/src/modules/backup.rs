use crate::modules::utils::calculate_total_size;
use crate::modules::{JChipVersion, NvramBackup, PsVersion};
use chrono::Utc;
use std::path::PathBuf;

// 生成随机备份信息
pub fn generate_backups() -> Vec<NvramBackup> {
    // 模拟游戏名称列表
    let game_names = [
        "HelloGame",
        "WorldAdventure",
        "SpaceExplorers",
        "OceanQuest",
        "MountainClimber",
    ];
    // 模拟PS版本列表
    let ps_versions = ["100A-001", "100A-002", "100B-001", "101A-001", "101A-002"];
    // 模拟JChip版本列表
    let jchip_versions = ["SGQLD001", "SGQLD002", "SGQLD003", "SGQLD004", "SGQLD005"];
    // 模拟PS构建版本
    let ps_builds = ["001", "002", "003", "004", "005"];
    // 模拟JChip构建版本
    let jchip_builds = ["001", "002", "003", "004", "005"];
    // 模拟文件列表
    let file_list = vec![
        "config.ini".to_string(),
        "data.bin".to_string(),
        "settings.json".to_string(),
        "metadata.txt".to_string(),
    ];

    let mut backups = Vec::new();

    // 生成10个随机备份
    for i in 1..=10 {
        // 随机选择各项数据
        let game_name = game_names[i % game_names.len()].to_string();

        // 创建PS版本信息
        let ps_version = PsVersion {
            version: ps_versions[i % ps_versions.len()].to_string(),
            build: ps_builds[i % ps_builds.len()].to_string(),
            path: PathBuf::from(r"C:\opt\ps").join(ps_versions[i % ps_versions.len()]),
        };

        // 创建JChip版本信息
        let jchip_version = JChipVersion {
            version: jchip_versions[i % jchip_versions.len()].to_string(),
            build: jchip_builds[i % jchip_builds.len()].to_string(),
            path: PathBuf::from(r"C:\opt\jchip").join(jchip_versions[i % jchip_versions.len()]),
        };

        // 计算文件总大小
        let total_size = calculate_total_size(&file_list);

        // 创建备份信息
        let backup = NvramBackup {
            id: i as u32,
            nvram_name: game_name,
            ps_version,
            jchip_version,
            backup_time: Utc::now(),
            file_list: file_list.clone(),
            total_size,
        };

        backups.push(backup);
    }

    backups
}

// 获取当前NVRAM信息
pub fn get_current_nvram_info(_nvram_path: &PathBuf) -> NvramBackup {
    // 模拟从磁盘读取NVRAM信息，实际实现时会遍历目录获取
    let nvram_name = "HelloGame"; // 模拟NVRAM名称

    // 创建PS版本信息
    let ps_version = PsVersion {
        version: "100A-001".to_string(),
        build: "001".to_string(),
        path: PathBuf::from(r"C:\opt\ps\100A-001"),
    };

    // 创建JChip版本信息
    let jchip_version = JChipVersion {
        version: "SGQLD001".to_string(),
        build: "001".to_string(),
        path: PathBuf::from(r"C:\opt\jchip\SGQLD001"),
    };

    // 模拟文件列表
    let file_list = vec![
        "config.ini".to_string(),
        "data.bin".to_string(),
        "settings.json".to_string(),
        "metadata.txt".to_string(),
        // 添加更多文件以模拟实际情况
        "save1.dat".to_string(),
        "save2.dat".to_string(),
    ];

    // 计算文件总大小
    let total_size = calculate_total_size(&file_list);

    // 返回NvramBackup结构体
    NvramBackup {
        id: 0,
        nvram_name: nvram_name.to_string(),
        ps_version,
        jchip_version,
        backup_time: Utc::now(), // 使用当前时间作为备份时间
        file_list,
        total_size,
    }
}
