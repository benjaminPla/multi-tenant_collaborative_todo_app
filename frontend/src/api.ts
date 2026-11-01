import type { Board, Member, Role, Team, Todo, TodoPatch, User } from './types.ts'

export class ApiError extends Error {
  status: number

  constructor(status: number, message: string) {
    super(message)
    this.status = status
  }
}

export const errorMessage = (err: unknown) => (err instanceof ApiError ? err.message : 'something went wrong')

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
  boards: {
    create: (teamId: string, name: string) => request<Board>('POST', `/teams/${teamId}/boards`, { name }),
    delete: (teamId: string, id: string) => request<void>('DELETE', `/teams/${teamId}/boards/${id}`),
    list:   (teamId: string) => request<Board[]>('GET', `/teams/${teamId}/boards`),
  },
  login:  (email: string, password: string) => request<void>('POST', '/auth/login', { email, password }),
  logout: () => request<void>('POST', '/auth/logout'),
  me:     () => request<User>('GET', '/auth/me'),
  members: {
    add:    (teamId: string, email: string, role: Role) => request<Member>('POST', `/teams/${teamId}/members`, { email, role }),
    list:   (teamId: string) => request<Member[]>('GET', `/teams/${teamId}/members`),
    remove: (teamId: string, userId: string) => request<void>('DELETE', `/teams/${teamId}/members/${userId}`),
    update: (teamId: string, userId: string, role: Role) => request<Member>('PATCH', `/teams/${teamId}/members/${userId}`, { role }),
  },
  teams: {
    create: (name: string) => request<Team>('POST', '/teams', { name }),
    delete: (id: string) => request<void>('DELETE', `/teams/${id}`),
    list:   () => request<Team[]>('GET', '/teams'),
  },
  todos: {
    create: (teamId: string, boardId: string, title: string) => request<Todo>('POST', `/teams/${teamId}/boards/${boardId}/todos`, { title }),
    delete: (teamId: string, boardId: string, id: string) => request<void>('DELETE', `/teams/${teamId}/boards/${boardId}/todos/${id}`),
    list:   (teamId: string, boardId: string) => request<Todo[]>('GET', `/teams/${teamId}/boards/${boardId}/todos`),
    update: (teamId: string, boardId: string, id: string, patch: TodoPatch) => request<Todo>('PATCH', `/teams/${teamId}/boards/${boardId}/todos/${id}`, patch),
  },
  users: {
    create: (email: string, password: string) => request<User>('POST', '/users', { email, password }),
    delete: (id: string) => request<void>('DELETE', `/users/${id}`),
    list:   () => request<User[]>('GET', '/users'),
  },
}
