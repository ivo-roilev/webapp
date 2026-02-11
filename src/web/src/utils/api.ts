import axios from 'axios';
import type {
  CreateUserPayload,
  CreateUserResponse,
  LoginPayload,
  LoginResponse,
  UserInfoResponse,
  ErrorResponse,
} from '../types';

const api = axios.create({
  baseURL: '/api',
  headers: {
    'Content-Type': 'application/json',
  },
});

export async function createUser(payload: CreateUserPayload): Promise<CreateUserResponse> {
  const response = await api.post<CreateUserResponse>('/users', payload);
  return response.data;
}

export async function login(payload: LoginPayload): Promise<LoginResponse> {
  const response = await api.post<LoginResponse>('/login', payload);
  return response.data;
}

export async function getUserInfo(userId: number): Promise<UserInfoResponse> {
  const response = await api.get<UserInfoResponse>(`/users/${userId}`);
  return response.data;
}

export function extractErrorMessage(error: unknown): string {
  if (axios.isAxiosError(error) && error.response?.data) {
    const data = error.response.data as ErrorResponse;
    if (data.message) {
      return data.message;
    }
  }
  if (error instanceof Error) {
    return error.message;
  }
  return 'An unexpected error occurred';
}
