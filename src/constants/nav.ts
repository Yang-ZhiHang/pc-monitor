type navItem = {
    label: string;
    icon: string;
    route: string;
}

export const title = "电脑应用使用统计";

export const navItems: navItem[] = ([
  { label: '统计概览', icon: 'bar-chart', route: 'dashboard' },
  { label: '导出数据', icon: 'download', route: 'export' },
  { label: '设置', icon: 'cog', route: 'settings' }
]);