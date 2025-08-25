type navItem = {
    label: string;
    icon: string;
    route: string;
}

export const title = "PUS";

export const navItems: navItem[] = ([
  { label: '概览', icon: 'chart-line', route: 'dashboard' },
  { label: '导出', icon: 'download', route: 'export' },
  { label: '设置', icon: 'cog', route: 'settings' }
]);