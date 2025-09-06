import { navItem } from "@/types/nav";

export const title = "PUS";

export const navItems: navItem[] = ([
  { label: 'nav.overview', detail: 'nav.detail.overview', icon: 'chart-line', route: 'dashboard' },
  { label: 'nav.export', detail: 'nav.detail.export', icon: 'file-export', route: 'export' },
  { label: 'nav.setting', detail: 'nav.detail.setting', icon: 'cog', route: 'settings' }
]);

export const githubLink = "https://github.com/Yang-ZhiHang/pc-monitor";