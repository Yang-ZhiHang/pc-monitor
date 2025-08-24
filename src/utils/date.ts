import { Dayjs } from 'dayjs';
import { periodLabel, Period } from '@/constants/dashboard';

// ===== week

/**
 * Get all dates in the current week (Monday to Sunday) based on the given date.
 * @param today
 * @returns [...'yyyy-mm-dd']
 */
const getWeekDays = (today: Dayjs): string[] => {
    const weekDates: string[] = [];
    // Obtain the current day of the week (0=Sunday, 1=Monday,...6=Saturday)
    const dayOfWeek = today.day();
    // Calculate the start of the week (Monday)
    const monday = today.subtract(dayOfWeek === 0 ? 6 : dayOfWeek - 1, 'day');
    for (let i = 0; i < 7; i++) {
        weekDates.push(monday.add(i, 'day').format('YYYY-MM-DD'));
    }
    return weekDates;
}

export const getWeekLabels = (): string[] => {
    return periodLabel[Period.WEEKLY];
}

export const getWeekData = (dataset: Record<string, number>, today: Dayjs): number[] => {
    const weekDates = getWeekDays(today);
    return weekDates.map(date => dataset[date] || 0);
}

// ===== month

/**
 * Group the days of the month into weeks.
 * @param today
 * @returns [[...'yyyy-mm-dd'], [...'yyyy-mm-dd'], ...]
 */
const getMonthWeekGroups = (today: Dayjs): string[][] => {
    const daysInMonth = today.daysInMonth();
    const weeks: string[][] = [];
    let week: string[] = [];
    for (let i = 1; i <= daysInMonth; ++i) {
        week.push(today.date(i).format('YYYY-MM-DD'));
        if (week.length === 7 || i === daysInMonth) {
            weeks.push(week);
            week = [];
        }
    }
    // Each element is an array of dates for that week
    return weeks;
}

export const getMonthWeekLabels = (today: Dayjs) => {
    const weeks = getMonthWeekGroups(today);
    return weeks.map((_, i) => `第${i + 1}周`);
}

export const getMonthWeekData = (dataset: Record<string, number>, today: Dayjs) => {
    const weeks = getMonthWeekGroups(today);
    return weeks.map(weekDates =>
        weekDates.reduce((sum, date) => sum + (dataset[date] || 0), 0)
    );
}

// ===== year

/**
 * Group the days of the years into months.
 * @param today
 * @returns [[...'yyyy-mm-dd'], [...'yyyy-mm-dd'], ...]
 */
const getYearMonthGroups = (today: Dayjs): string[][] => {
    const months: string[][] = [];
    const startOfYear = today.startOf('year');

    for (let i = 1; i <= 12; i++) {
        const monthStart = startOfYear.month(i - 1).startOf('month');
        const monthEnd = monthStart.endOf('month');
        const monthDays: string[] = [];
        for (let day = monthStart.date(); day <= monthEnd.date(); day++) {
            monthDays.push(monthStart.date(day).format('YYYY-MM-DD'));
        }
        months.push(monthDays);
    }
    return months;
}

export const getYearMonthLabels = (): string[] => {
    return periodLabel[Period.YEARLY];
}

export const getYearMonthData = (dataset: Record<string, number>, today: Dayjs): number[] => {
    const months = getYearMonthGroups(today);
    return months.map(monthDays =>
        monthDays.reduce((sum, date) => sum + (dataset[date] || 0), 0)
    );
}