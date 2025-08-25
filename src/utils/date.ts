import dayjs from 'dayjs';
import { periodLabel, Period } from '@/constants/dashboard';

// ===== week

/**
 * Get all dates in the appointed week (Monday to Sunday) based on the given date.
 * @param today
 * @returns [...'yyyy-mm-dd']
 */
const getWeekDays = (sub: number = 0): string[] => {
    const today = dayjs();
    const weekDates: string[] = [];
    // Obtain the current day of the week (0=Sunday, 1=Monday,...6=Saturday)
    const dayOfWeek = today.day();
    // Calculate the start of the week (Monday)
    const monday = today.subtract(dayOfWeek === 0 ? 6 : dayOfWeek - 1, 'day').subtract(sub, 'week');
    for (let i = 0; i < 7; i++) {
        weekDates.push(monday.add(i, 'day').format('YYYY-MM-DD'));
    }
    return weekDates;
}

export const getWeekLabels = (): string[] => {
    return periodLabel[Period.WEEKLY];
}

export const getWeekData = (dataset: Record<string, number>, sub: number = 0): number[] => {
    const weekDates = getWeekDays(sub);
    return weekDates.map(date => dataset[date] || 0);
}

export const getWeekDataSum = (dataset: Record<string, number>, sub: number = 0): number => {
    const weekDates = getWeekDays(sub);
    return weekDates.reduce((sum, date) => sum + (dataset[date] || 0), 0);
}

export const getWeekDataAvg = (dataset: Record<string, number>, sub: number = 0): number => {
    const weekDates = getWeekDays(sub);
    const today = dayjs().format('YYYY-MM-DD');
    // Only consider dates up to today
    const validDates = weekDates.filter(date => date <= today);
    const weekData = validDates.map(date => dataset[date] || 0);
    const sum = weekData.reduce((acc, val) => acc + val, 0);
    return validDates.length > 0 ? Math.round(sum / validDates.length) : 0;
}

// ===== month

/**
 * Group the days of the month into weeks.
 * @param today
 * @returns [[...'yyyy-mm-dd'], [...'yyyy-mm-dd'], ...]
 */
const getMonthWeekGroups = (): string[][] => {
    const today = dayjs();
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

export const getMonthWeekLabels = () => {
    const weeks = getMonthWeekGroups();
    return weeks.map((_, i) => `第${i + 1}周`);
}

export const getMonthWeekData = (dataset: Record<string, number>) => {
    const weeks = getMonthWeekGroups();
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
const getYearMonthGroups = (): string[][] => {
    const months: string[][] = [];
    const startOfYear = dayjs().startOf('year');

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

export const getYearMonthData = (dataset: Record<string, number>): number[] => {
    const months = getYearMonthGroups();
    return months.map(monthDays => monthDays.reduce((sum, date) => sum + (dataset[date] || 0), 0));
}

// ===== today and yesterday

export const getCmpDays = (dataset: Record<string, number>): [number, number] => {
    const today = dayjs().format('YYYY-MM-DD');
    const yesterday = dayjs().subtract(1, 'day').format('YYYY-MM-DD');
    return [dataset[today] || 0, dataset[yesterday] || 0];
}

export const getCurrentDate = (sub: number = 0): string => {
    return dayjs().subtract(sub, 'day').format('YYYY-MM-DD');
}