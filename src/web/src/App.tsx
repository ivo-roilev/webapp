import { BrowserRouter, Routes, Route, Navigate } from 'react-router-dom';
import { Layout } from './components/Layout';
import { CreateUserPage } from './pages/CreateUserPage';
import { LoginPage } from './pages/LoginPage';
import { UserInfoPage } from './pages/UserInfoPage';
import './App.css';

function App() {
  return (
    <BrowserRouter>
      <Layout>
        <Routes>
          <Route path="/create-user" element={<CreateUserPage />} />
          <Route path="/login" element={<LoginPage />} />
          <Route path="/user-info" element={<UserInfoPage />} />
          <Route path="*" element={<Navigate to="/login" replace />} />
        </Routes>
      </Layout>
    </BrowserRouter>
  );
}

export default App;
