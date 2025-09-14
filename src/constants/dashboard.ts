import { Period } from "@/types/dashboard";

export const chartTitle: Record<string, string> = {
    dailyUsage: 'dashboard.chart.0',
    appUsage: 'dashboard.chart.1'
}

export const periodText: Record<Period, string> = {
    [Period.WEEKLY]: 'dashboard.period.week',
    [Period.MONTHLY]: 'dashboard.period.month',
    [Period.YEARLY]: 'dashboard.period.year'
};

export const periodLabelKey: Record<Period, string[]> = {
    [Period.WEEKLY]: [ 'week.sun', 'week.mon', 'week.tue', 'week.wed', 'week.thu', 'week.fri', 'week.sat'],
    [Period.MONTHLY]: ['week.0', 'week.1', 'week.2', 'week.3', 'week.4'],
    [Period.YEARLY]: ['month.jan', 'month.feb', 'month.mar', 'month.apr', 'month.may', 'month.jun', 'month.jul', 'month.aug', 'month.sep', 'month.oct', 'month.nov', 'month.dec']
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