export const chartTitle: Record<string, string> = {
    dailyUsage: '每日电脑使用时长/小时',
    appUsage: '今日 TOP4 应用使用占比'
}

export enum Period {
    WEEKLY = 'weekly',
    MONTHLY = 'monthly',
    YEARLY = 'yearly'
}

export type PeriodType =
    | Period.WEEKLY
    | Period.MONTHLY
    | Period.YEARLY;

export const periodText: Record<PeriodType, string> = {
    [Period.WEEKLY]: '周',
    [Period.MONTHLY]: '月',
    [Period.YEARLY]: '年'
};

export const periodLabel: Record<PeriodType, string[]> = {
    [Period.WEEKLY]: ['周一', '周二', '周三', '周四', '周五', '周六', '周日'],
    [Period.MONTHLY]: ['1日', '5日', '10日', '15日', '20日', '25日', '30日'],
    [Period.YEARLY]: ['1月', '2月', '3月', '4月', '5月', '6月', '7月', '8月', '9月', '10月', '11月', '12月']
};

export const compareCardInfo: Array<{ title: string; icon: string; bgColor: string }> = [
    {
        title: '本周总使用',
        icon: 'fa-solid fa-calendar-week',
        bgColor: 'bg-primary/20 text-primary'
    },
    {
        title: '活跃应用',
        icon: 'fa-solid fa-chart-line',
        bgColor: 'bg-purple-500/20 text-purple-400'
    },
    {
        title: '平均每日使用',
        icon: 'fa-solid fa-calendar-day',
        bgColor: 'bg-orange-500/20 text-orange-400'
    }
]