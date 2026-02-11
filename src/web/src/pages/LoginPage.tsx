import { useState } from 'react';
import type { FormEvent } from 'react';
import { useNavigate } from 'react-router-dom';
import { InputField, Button, ErrorMessage } from '../components/FormComponents';
import { login, extractErrorMessage } from '../utils/api';
import { storeUserId } from '../utils/storage';

interface FormErrors {
  username?: string;
  password?: string;
}

export function LoginPage() {
  const navigate = useNavigate();

  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');

  const [errors, setErrors] = useState<FormErrors>({});
  const [apiError, setApiError] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);

  function validate(): FormErrors {
    const errs: FormErrors = {};
    if (!username.trim()) errs.username = 'Username is required';
    if (!password.trim()) errs.password = 'Password is required';
    return errs;
  }

  async function handleSubmit(e: FormEvent) {
    e.preventDefault();
    setApiError(null);

    const validationErrors = validate();
    setErrors(validationErrors);
    if (Object.keys(validationErrors).length > 0) return;

    setLoading(true);
    try {
      const response = await login({
        username: username.trim(),
        password: password.trim(),
      });
      storeUserId(response.user_id);
      navigate('/user-info');
    } catch (error) {
      setApiError(extractErrorMessage(error));
    } finally {
      setLoading(false);
    }
  }

  return (
    <div className="page">
      <h1>Login</h1>
      <form onSubmit={handleSubmit} noValidate>
        <InputField
          label="Username"
          value={username}
          onChange={(e) => setUsername(e.target.value)}
          error={errors.username}
          required
        />
        <InputField
          label="Password"
          type="password"
          value={password}
          onChange={(e) => setPassword(e.target.value)}
          error={errors.password}
          required
        />
        <ErrorMessage message={apiError} />
        <Button type="submit" disabled={loading}>
          {loading ? 'Logging in...' : 'Login'}
        </Button>
      </form>
    </div>
  );
}
