export const secondsToDurationString = (seconds: number): string => {
  let hours = 0;
  let minutes = 0;

  while (seconds >= 60) {
    seconds -= 60;
    minutes += 1;

    if (minutes >= 60) {
      minutes -= 60;
      hours += 1;
    }
  }

  if (hours === 0 && minutes === 0) {
    return '0 minutes';
  }

  if (hours === 0) {
    return `${minutes} minute${minutes > 1 ? 's' : ''}`;
  }

  if (minutes === 0) {
    return `${hours} hour${hours > 1 ? 's' : ''}`;
  }

  return `${hours} hour${hours > 1 ? 's' : ''} ${minutes} minute${minutes > 1 ? 's' : ''}`;
};