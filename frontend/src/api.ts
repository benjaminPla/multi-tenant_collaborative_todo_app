import type { User } from './types.ts'

export class ApiError extends Error {
  status: number

  constructor(status: number, message: string) {
    super(message)
    this.status = status
  }
}

async function request<T>(method: string, path: string, body?: unknown): Promise<T> {
  const response = await fetch(`/api${path}`, {
    body:        body === undefined ? undefined : JSON.stringify(body),
    credentials: 'same-origin',
    headers:     body === undefined ? undefined : { 'content-type': 'application/json' },
    method,
  })
  if (!response.ok) {
    const payload = await response.json().catch(() => null)
    throw new ApiError(response.status, payload?.error ?? response.statusText)
  }
  return response.status === 204 ? (undefined as T) : response.json()
}

export const api = {
  login:  (email: string, password: string) => request<void>('POST', '/auth/login', { email, password }),
  logout: () => request<void>('POST', '/auth/logout'),
  me:     () => request<User>('GET', '/auth/me'),
}
