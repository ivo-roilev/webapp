import type { ReactNode } from 'react';
import { Link } from 'react-router-dom';

interface LayoutProps {
  children: ReactNode;
}

export function Layout({ children }: LayoutProps) {
  return (
    <div className="app-layout">
      <header className="app-header">
        <nav>
          <Link to="/create-user">Create User</Link>
          <Link to="/login">Login</Link>
        </nav>
      </header>
      <main className="app-main">{children}</main>
    </div>
  );
}
