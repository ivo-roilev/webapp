import { useState } from 'react';
import type { FormEvent } from 'react';
import { useNavigate } from 'react-router-dom';
import { InputField, Button, ErrorMessage } from '../components/FormComponents';
import { createUser, extractErrorMessage } from '../utils/api';
import { storeUserId } from '../utils/storage';

interface FormErrors {
  username?: string;
  password?: string;
}

export function CreateUserPage() {
  const navigate = useNavigate();

  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const [firstName, setFirstName] = useState('');
  const [lastName, setLastName] = useState('');
  const [email, setEmail] = useState('');
  const [title, setTitle] = useState('');
  const [hobby, setHobby] = useState('');

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
      const response = await createUser({
        username: username.trim(),
        password: password.trim(),
        first_name: firstName.trim() || undefined,
        last_name: lastName.trim() || undefined,
        email: email.trim() || undefined,
        title: title.trim() || undefined,
        hobby: hobby.trim() || undefined,
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
      <h1>Create User</h1>
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
        <InputField
          label="First Name"
          value={firstName}
          onChange={(e) => setFirstName(e.target.value)}
        />
        <InputField
          label="Last Name"
          value={lastName}
          onChange={(e) => setLastName(e.target.value)}
        />
        <InputField
          label="Email"
          type="email"
          value={email}
          onChange={(e) => setEmail(e.target.value)}
        />
        <InputField
          label="Title"
          value={title}
          onChange={(e) => setTitle(e.target.value)}
        />
        <InputField
          label="Hobby"
          value={hobby}
          onChange={(e) => setHobby(e.target.value)}
        />
        <ErrorMessage message={apiError} />
        <Button type="submit" disabled={loading}>
          {loading ? 'Creating...' : 'Create User'}
        </Button>
      </form>
    </div>
  );
}
