import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/react';
import App from '../App';

describe('App', () => {
  it('renders the navigation links', () => {
    render(<App />);

    const navLinks = screen.getAllByRole('link');
    const linkTexts = navLinks.map((l) => l.textContent);
    expect(linkTexts).toContain('Create User');
    expect(linkTexts).toContain('Login');
  });

  it('defaults to the login page', () => {
    render(<App />);

    // The default route redirects to /login which renders the Login heading
    expect(screen.getByRole('heading', { name: 'Login' })).toBeInTheDocument();
  });
});

