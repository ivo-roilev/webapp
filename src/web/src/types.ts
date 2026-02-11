// API response types matching the Rust service

export interface CreateUserPayload {
  username: string;
  password: string;
  first_name?: string;
  last_name?: string;
  email?: string;
  title?: string;
  hobby?: string;
}

export interface CreateUserResponse {
  user_id: number;
}

export interface LoginPayload {
  username: string;
  password: string;
}

export interface LoginResponse {
  user_id: number;
}

export interface UserInfoResponse {
  id: number;
  username: string;
  first_name: string | null;
  last_name: string | null;
  email: string | null;
  title: string | null;
  hobby: string | null;
}

export interface ErrorResponse {
  error: string;
  message: string;
}
