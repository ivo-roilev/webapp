import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { MemoryRouter } from 'react-router-dom';
import { CreateUserPage } from '../pages/CreateUserPage';

function renderWithRouter(ui: React.ReactElement, { route = '/' } = {}) {
  return render(<MemoryRouter initialEntries={[route]}>{ui}</MemoryRouter>);
}

describe('CreateUserPage', () => {
  it('renders the create user form with all fields', () => {
    renderWithRouter(<CreateUserPage />);

    expect(screen.getByRole('heading', { name: 'Create User' })).toBeInTheDocument();
    expect(screen.getByLabelText('Username')).toBeInTheDocument();
    expect(screen.getByLabelText('Password')).toBeInTheDocument();
    expect(screen.getByLabelText('First Name')).toBeInTheDocument();
    expect(screen.getByLabelText('Last Name')).toBeInTheDocument();
    expect(screen.getByLabelText('Email')).toBeInTheDocument();
    expect(screen.getByLabelText('Title')).toBeInTheDocument();
    expect(screen.getByLabelText('Hobby')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Create User' })).toBeInTheDocument();
  });

  it('shows validation errors when submitting with empty required fields', async () => {
    const user = userEvent.setup();
    renderWithRouter(<CreateUserPage />);

    await user.click(screen.getByRole('button', { name: 'Create User' }));

    expect(screen.getByText('Username is required')).toBeInTheDocument();
    expect(screen.getByText('Password is required')).toBeInTheDocument();
  });

  it('shows validation error only for password when username is filled', async () => {
    const user = userEvent.setup();
    renderWithRouter(<CreateUserPage />);

    await user.type(screen.getByLabelText('Username'), 'testuser');
    await user.click(screen.getByRole('button', { name: 'Create User' }));

    expect(screen.queryByText('Username is required')).not.toBeInTheDocument();
    expect(screen.getByText('Password is required')).toBeInTheDocument();
  });

  it('shows validation error only for username when password is filled', async () => {
    const user = userEvent.setup();
    renderWithRouter(<CreateUserPage />);

    await user.type(screen.getByLabelText('Password'), 'testpass');
    await user.click(screen.getByRole('button', { name: 'Create User' }));

    expect(screen.getByText('Username is required')).toBeInTheDocument();
    expect(screen.queryByText('Password is required')).not.toBeInTheDocument();
  });
});
