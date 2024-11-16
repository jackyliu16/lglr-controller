#[derive(Debug)]
pub enum CurrentScreen {
    InstallFleetInfo,        // 输入舰队信息
    InstallTargetInfo,       // 输入目标信息
    SelectTargetAndFleet,    // 选择在一次袭击中的舰队和目标
    FleetRunningTimeScreen,  // 简单的显示各舰队运行时间
    FleetDepartureCountdown, // 显示倒计时并且在结束之后更新舰队位置
    ConfirmedExitScreen,     // 询问是否确认退出
}

// [Tab]      InstallFleetInfo/InstallFleetInfo switch between fleet/target list
// [Enter/E]  InstallFleetInfo/InstallFleetInfo enable editing
// [Ctrl+N]   InstallFleetInfo -> InstallGoalInfo -> SelectTargetAndFleet -> FleetRunningTimeScreen -> ConfirmedInfoScreen -> SelectTargetAndFleet
// [Ctrl+Left/Right]    SelectTargetAndFleet: Switching between two block in screen to select the fleet and goal right now
// [Enter]              SelectTargetAndFleet: Switching this item

// Screen Switching Diagram
//         InstallFleetInfo       ──┬─► ConfirmedExitScreen
//              ▲   │ [Tab,Enter/e] │
//     Ctrl + P │   │               │
//              │   │ Ctrl + N      │ q
//              │   ▼               │
//         InstallTargetInfo      ──┤
//              ▲   │ [Tab,Enter,e] │
//     Ctrl + P │   │               │
//              │   │ Ctrl + N      │
//              │   ▼               │
//  ┌─────►SelectTargetAndFleet   ──┤
//  │               │ [Ctrl+R/L]    │
//  │ Ctrl + N      │ Ctrl + N      │
//  │               ▼               │
//  │      FleetRunningTimeScreen ──┤
//  │               │               │
//  │               │ Ctrl + N      │
//  │               ▼               │
//  └──────ConfirmedExitScreen    ──┘
