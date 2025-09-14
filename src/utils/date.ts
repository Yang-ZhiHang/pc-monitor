import dayjs from 'dayjs';
import { periodLabelKey } from '@/constants/dashboard';
import { Period } from '@/types/dashboard';
import { Mode } from '@/types/date';

// ===== week

/**
 * Get all dates in the appointed week (Monday to Sunday) based on the given date.
 * @param today
 * @returns [...'yyyy-mm-dd']
 */
const getWeekDays = (mode: Mode = Mode.NOP, n: number = 0): string[] => {
    const today = dayjs();
    const weekDates: string[] = [];
    // Obtain the current day of the week (0=Sunday, 1=Monday,...6=Saturday)
    // const dayOfWeek = today.day();
    // Calculate the start of the week (Monday)
    // let monday = today.subtract(dayOfWeek === 0 ? 6 : dayOfWeek - 1, 'day');
    let startOfWeek = today.startOf('week'); // Sunday
    switch (mode) {
        case Mode.SUB:
            startOfWeek = startOfWeek.subtract(Math.abs(n), 'week');
            break;
        case Mode.INC:
            startOfWeek = startOfWeek.add(Math.abs(n), 'week');
            break;
        default:
            break;
    }
    for (let i = 0; i < 7; i++) {
        weekDates.push(startOfWeek.add(i, 'day').format('YYYY-MM-DD'));
    }
    return weekDates;
}

export const getWeekLabelKey = (): string[] => {
    return periodLabelKey[Period.WEEKLY];
}

export const getWeekData = (dataset: Record<string, number>, mode: Mode = Mode.NOP, n: number = 0): number[] => {
    const weekDates = getWeekDays(mode, n);
    return weekDates.map(date => dataset[date] || 0);
}

export const getWeekDataSum = (dataset: Record<string, number>, mode: Mode = Mode.NOP, n: number = 0): number => {
    const weekDates = getWeekDays(mode, n);
    return weekDates.reduce((sum, date) => sum + (dataset[date] || 0), 0);
}

export const getWeekDataAvg = (dataset: Record<string, number>, mode: Mode = Mode.NOP, n: number = 0): number => {
    const weekDates = getWeekDays(mode, n);
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
    const weeks: string[][] = [];
    const today = dayjs();
    const startOfMonth = today.startOf('month');
    const endOfMonth = today.endOf('month');
    let current = startOfMonth.startOf('week'); // Start from the first day of the week containing the 1st
    while (current.isBefore(endOfMonth.endOf('week')) || current.isSame(endOfMonth.endOf('week'), 'day')) {
        const week = [];
        for (let i = 0; i < 7; i++) {
            week.push(current.format('YYYY-MM-DD'));
            current = current.add(1, 'day');
        }
        weeks.push(week);
    }
    // Each element is an array of dates for that week
    return weeks;
}

export const getMonthWeekLabelKey = (): string[] => {
    return periodLabelKey[Period.MONTHLY];
}

export const getMonthWeekData = (dataset: Record<string, number>) => {
    const weeks = getMonthWeekGroups();
    console.log('Month week groups:', weeks);
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

export const getYearMonthLabelKey = (): string[] => {
    return periodLabelKey[Period.YEARLY];
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