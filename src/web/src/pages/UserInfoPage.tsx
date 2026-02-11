import { useEffect, useState } from 'react';
import { Link } from 'react-router-dom';
import { ErrorMessage, Button } from '../components/FormComponents';
import { getUserInfo, extractErrorMessage } from '../utils/api';
import { getUserId } from '../utils/storage';

export function UserInfoPage() {
  const [userGreeting, setUserGreeting] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);

  const userId = getUserId();

  async function fetchUserInfo(id: number) {
    setLoading(true);
    setError(null);
    try {
      const greeting = await getUserInfo(id);
      setUserGreeting(greeting);
    } catch (err) {
      setError(extractErrorMessage(err));
    } finally {
      setLoading(false);
    }
  }

  useEffect(() => {
    if (userId !== null) {
      fetchUserInfo(userId);
    } else {
      setLoading(false);
    }
  }, [userId]);

  if (userId === null) {
    return (
      <div className="page center">
        <h1>Greetings!</h1>
        <p>No user session found.</p>
        <p>
          Please <Link to="/login">login</Link> or{' '}
          <Link to="/create-user">create a user</Link> first.
        </p>
      </div>
    );
  }

  if (loading) {
    return (
      <div className="page center">
        <h1>Greetings!</h1>
        <p>Loading...</p>
      </div>
    );
  }

  if (error) {
    return (
      <div className="page center">
        <h1>Greetings!</h1>
        <ErrorMessage message={error} />
        <Button onClick={() => fetchUserInfo(userId)}>Retry</Button>
        <p>
          Or go back to <Link to="/login">login</Link> or{' '}
          <Link to="/create-user">create a user</Link>.
        </p>
      </div>
    );
  }

  if (!userGreeting) {
    return null;
  }

  return (
    <div className="page center">
      <h1>Greetings!</h1>
      <div className="user-info">
        <p className="user-greeting">{userGreeting}</p>
      </div>
    </div>
  );
}
