import type { ReactNode, InputHTMLAttributes } from 'react';

interface InputFieldProps extends InputHTMLAttributes<HTMLInputElement> {
  label: string;
  error?: string;
}

export function InputField({ label, error, id, ...props }: InputFieldProps) {
  const inputId = id ?? label.toLowerCase().replace(/\s+/g, '-');
  return (
    <div className="form-field">
      <label htmlFor={inputId}>{label}</label>
      <input id={inputId} {...props} className={error ? 'input-error' : ''} />
      {error && <span className="field-error">{error}</span>}
    </div>
  );
}

interface ButtonProps {
  children: ReactNode;
  type?: 'button' | 'submit';
  disabled?: boolean;
  onClick?: () => void;
}

export function Button({ children, type = 'button', disabled, onClick }: ButtonProps) {
  return (
    <button type={type} disabled={disabled} onClick={onClick} className="btn">
      {children}
    </button>
  );
}

interface ErrorMessageProps {
  message: string | null;
}

export function ErrorMessage({ message }: ErrorMessageProps) {
  if (!message) return null;
  return <div className="error-message">{message}</div>;
}
