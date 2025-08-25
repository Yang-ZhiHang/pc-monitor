export const chartTitle: Record<string, string> = {
    dailyUsage: 'dashboard.chart.0',
    appUsage: 'dashboard.chart.1'
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
    [Period.WEEKLY]: 'dashboard.period.week',
    [Period.MONTHLY]: 'dashboard.period.month',
    [Period.YEARLY]: 'dashboard.period.year'
};

export const periodLabel: Record<PeriodType, string[]> = {
    [Period.WEEKLY]: ['周一', '周二', '周三', '周四', '周五', '周六', '周日'],
    [Period.MONTHLY]: ['1日', '5日', '10日', '15日', '20日', '25日', '30日'],
    [Period.YEARLY]: ['1月', '2月', '3月', '4月', '5月', '6月', '7月', '8月', '9月', '10月', '11月', '12月']
};

export const compareCardInfo: Array<{ title: string; icon: string; bgColor: string; cmpText: string }> = [
    {
        title: 'dashboard.cmp-card.0.title',
        icon: 'calendar-week',
        bgColor: 'bg-primary/20 text-primary',
        cmpText: 'dashboard.cmp-card.0.cmpText'
    },
    {
        title: 'dashboard.cmp-card.1.title',
        icon: 'puzzle-piece',
        bgColor: 'bg-purple-500/20 text-purple-400',
        cmpText: 'dashboard.cmp-card.1.cmpText'
    },
    {
        title: 'dashboard.cmp-card.2.title',
        icon: 'calendar-day',
        bgColor: 'bg-orange-500/20 text-orange-400',
        cmpText: 'dashboard.cmp-card.2.cmpText'
    }
]