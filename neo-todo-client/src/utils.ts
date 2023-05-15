
export function taskKind(kind: number): string {
  if (kind === 0) {
    return 'Common Task'
  } else if (kind === 1) {
    return 'Duration Task'
  } else if (kind === 2) {
    return 'Reminder Task'
  }
  return ''
}

export function taskIcon(kind: number): string {
  if (kind === 0) {
    return 'mdi-timelapse'
  } else if (kind === 1) {
    return 'mdi-timer-sand'
  } else if (kind === 2) {
    return 'mdi-alert-box-outline'
  }
  return 'mdi-altimeter'
}