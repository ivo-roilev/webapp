import { describe, it, expect, beforeEach, vi } from 'vitest';
import { render, screen } from '@testing-library/react';
import { MemoryRouter } from 'react-router-dom';
import { UserInfoPage } from '../pages/UserInfoPage';

// Mock the API module to prevent real HTTP requests
vi.mock('../utils/api', () => ({
  getUserInfo: vi.fn(() => new Promise(() => { })), // never resolves — keeps loading state
  extractErrorMessage: vi.fn((e: unknown) => String(e)),
}));

function renderWithRouter(ui: React.ReactElement, { route = '/' } = {}) {
  return render(<MemoryRouter initialEntries={[route]}>{ui}</MemoryRouter>);
}

describe('UserInfoPage', () => {
  beforeEach(() => {
    localStorage.clear();
  });

  it('shows redirect links when no user_id in storage', () => {
    renderWithRouter(<UserInfoPage />);

    expect(screen.getByText('No user session found.')).toBeInTheDocument();
    expect(screen.getByText('login')).toBeInTheDocument();
    expect(screen.getByText('create a user')).toBeInTheDocument();
  });

  it('shows loading state when user_id exists', () => {
    localStorage.setItem('user_id', '1');
    renderWithRouter(<UserInfoPage />);

    expect(screen.getByText('Loading...')).toBeInTheDocument();
  });
});
