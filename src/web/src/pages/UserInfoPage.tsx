import { useEffect, useState } from 'react';
import { Link } from 'react-router-dom';
import { ErrorMessage, Button } from '../components/FormComponents';
import { getUserInfo, extractErrorMessage } from '../utils/api';
import { getUserId } from '../utils/storage';
import type { UserInfoResponse } from '../types';

export function UserInfoPage() {
  const [userInfo, setUserInfo] = useState<UserInfoResponse | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);

  const userId = getUserId();

  async function fetchUserInfo(id: number) {
    setLoading(true);
    setError(null);
    try {
      const data = await getUserInfo(id);
      setUserInfo(data);
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
        <h1>User Info</h1>
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
        <h1>User Info</h1>
        <p>Loading...</p>
      </div>
    );
  }

  if (error) {
    return (
      <div className="page center">
        <h1>User Info</h1>
        <ErrorMessage message={error} />
        <Button onClick={() => fetchUserInfo(userId)}>Retry</Button>
        <p>
          Or go back to <Link to="/login">login</Link> or{' '}
          <Link to="/create-user">create a user</Link>.
        </p>
      </div>
    );
  }

  if (!userInfo) {
    return null;
  }

  return (
    <div className="page center">
      <h1>User Info</h1>
      <div className="user-info">
        <div className="info-row">
          <span className="info-label">Username:</span>
          <span className="info-value">{userInfo.username}</span>
        </div>
        {userInfo.first_name && (
          <div className="info-row">
            <span className="info-label">First Name:</span>
            <span className="info-value">{userInfo.first_name}</span>
          </div>
        )}
        {userInfo.last_name && (
          <div className="info-row">
            <span className="info-label">Last Name:</span>
            <span className="info-value">{userInfo.last_name}</span>
          </div>
        )}
        {userInfo.email && (
          <div className="info-row">
            <span className="info-label">Email:</span>
            <span className="info-value">{userInfo.email}</span>
          </div>
        )}
        {userInfo.title && (
          <div className="info-row">
            <span className="info-label">Title:</span>
            <span className="info-value">{userInfo.title}</span>
          </div>
        )}
        {userInfo.hobby && (
          <div className="info-row">
            <span className="info-label">Hobby:</span>
            <span className="info-value">{userInfo.hobby}</span>
          </div>
        )}
      </div>
    </div>
  );
}
