import { describe, it, expect, beforeEach } from 'vitest';
import { storeUserId, getUserId, clearUserId } from '../utils/storage';

describe('storage utility', () => {
  beforeEach(() => {
    localStorage.clear();
  });

  describe('storeUserId', () => {
    it('stores a user_id in localStorage', () => {
      storeUserId(42);
      expect(localStorage.getItem('user_id')).toBe('42');
    });

    it('overwrites an existing user_id', () => {
      storeUserId(1);
      storeUserId(2);
      expect(localStorage.getItem('user_id')).toBe('2');
    });
  });

  describe('getUserId', () => {
    it('returns null when no user_id is stored', () => {
      expect(getUserId()).toBeNull();
    });

    it('returns the stored user_id as a number', () => {
      localStorage.setItem('user_id', '42');
      expect(getUserId()).toBe(42);
    });

    it('returns null for non-numeric stored value', () => {
      localStorage.setItem('user_id', 'abc');
      expect(getUserId()).toBeNull();
    });
  });

  describe('clearUserId', () => {
    it('removes user_id from localStorage', () => {
      storeUserId(42);
      clearUserId();
      expect(getUserId()).toBeNull();
    });

    it('does not throw when no user_id exists', () => {
      expect(() => clearUserId()).not.toThrow();
    });
  });
});
