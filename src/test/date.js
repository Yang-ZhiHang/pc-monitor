import dayjs from 'dayjs';

const weeks = [];
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

console.log(weeks);