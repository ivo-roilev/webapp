import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { MemoryRouter } from 'react-router-dom';
import { LoginPage } from '../pages/LoginPage';

function renderWithRouter(ui: React.ReactElement, { route = '/' } = {}) {
  return render(<MemoryRouter initialEntries={[route]}>{ui}</MemoryRouter>);
}

describe('LoginPage', () => {
  it('renders the login form with username and password fields', () => {
    renderWithRouter(<LoginPage />);

    expect(screen.getByRole('heading', { name: 'Login' })).toBeInTheDocument();
    expect(screen.getByLabelText('Username')).toBeInTheDocument();
    expect(screen.getByLabelText('Password')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Login' })).toBeInTheDocument();
  });

  it('shows validation errors when submitting with empty fields', async () => {
    const user = userEvent.setup();
    renderWithRouter(<LoginPage />);

    await user.click(screen.getByRole('button', { name: 'Login' }));

    expect(screen.getByText('Username is required')).toBeInTheDocument();
    expect(screen.getByText('Password is required')).toBeInTheDocument();
  });

  it('clears username validation error when username is provided', async () => {
    const user = userEvent.setup();
    renderWithRouter(<LoginPage />);

    await user.type(screen.getByLabelText('Username'), 'testuser');
    await user.click(screen.getByRole('button', { name: 'Login' }));

    expect(screen.queryByText('Username is required')).not.toBeInTheDocument();
    expect(screen.getByText('Password is required')).toBeInTheDocument();
  });
});
