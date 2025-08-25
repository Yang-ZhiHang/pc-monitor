type navItem = {
    label: string;
    icon: string;
    route: string;
}

export const title = "PUS";

export const navItems: navItem[] = ([
  { label: 'nav.overview', icon: 'chart-line', route: 'dashboard' },
  { label: 'nav.export', icon: 'download', route: 'export' },
  { label: 'nav.setting', icon: 'cog', route: 'settings' }
]);