export const format_seconds = (sec: number) => {
  let h = Math.floor(sec / 3600);
  let m = Math.floor((sec % 3600) / 60);
  let s = Math.floor(sec % 60);
  if (h == 0 && m == 0) {
    return `${s}s`;
  } else if (h == 0) {
    return `${m}m${s}s`;
  }
  return `${h}h ${m}m ${s}s`;
}
