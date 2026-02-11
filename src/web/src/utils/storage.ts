const USER_ID_KEY = 'user_id';

export function storeUserId(userId: number): void {
  localStorage.setItem(USER_ID_KEY, String(userId));
}

export function getUserId(): number | null {
  const value = localStorage.getItem(USER_ID_KEY);
  if (value === null) return null;
  const parsed = parseInt(value, 10);
  return isNaN(parsed) ? null : parsed;
}

export function clearUserId(): void {
  localStorage.removeItem(USER_ID_KEY);
}
