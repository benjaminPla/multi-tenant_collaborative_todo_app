import { api } from '../api.ts'
import { EmptyState } from '../components/EmptyState.tsx'
import { Page } from '../components/Page.tsx'
import { formatDate } from '../format.ts'
import { useAction, useLoad } from '../hooks.ts'
import type { FormEvent } from 'react'

export function Users() {
  const users  = useLoad(api.users.list)
  const action = useAction(users.reload)

  const create = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault()
    const form = event.currentTarget
    const data = new FormData(form)
    if (await action.run(() => api.users.create(String(data.get('email')), String(data.get('password'))))) {
      form.reset()
    }
  }

  return (
    <Page
      actions={
        <form className="inline-form" onSubmit={create}>
          <input aria-label="Email" autoComplete="off" className="field__input" name="email" placeholder="Email" required type="email" />
          <input aria-label="Password" autoComplete="new-password" className="field__input" name="password" placeholder="Password" required type="password" />
          <button className="button button--primary" type="submit">New user</button>
        </form>
      }
      error={action.error ?? users.error}
      title="Users"
    >
      {users.data?.length === 0 && <EmptyState>No users yet</EmptyState>}
      {!!users.data?.length && (
        <table className="table">
          <thead>
            <tr>
              <th>Email</th>
              <th>Admin</th>
              <th>Created</th>
              <th />
            </tr>
          </thead>
          <tbody>
            {users.data.map((user) => (
              <tr key={user.id}>
                <td>{user.email}</td>
                <td>{user.is_admin ? 'yes' : 'no'}</td>
                <td>{formatDate(user.created_at)}</td>
                <td className="table__actions">
                  <button className="button button--danger" onClick={() => action.run(() => api.users.delete(user.id))} type="button">
                    Delete
                  </button>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </Page>
  )
}
