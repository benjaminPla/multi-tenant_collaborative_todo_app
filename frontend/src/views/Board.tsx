import { api } from '../api.ts'
import { EmptyState } from '../components/EmptyState.tsx'
import { Page } from '../components/Page.tsx'
import { useAction, useLoad } from '../hooks.ts'
import type { Board as BoardType, Todo, TodoPatch, TodoStatus } from '../types.ts'
import { useCallback, type FormEvent } from 'react'

type Props = {
  board:  BoardType
  onBack: () => void
}

const STATUSES: TodoStatus[] = ['todo', 'in-progress', 'done']

export function Board({ board, onBack }: Props) {
  const todos   = useLoad(useCallback(() => api.todos.list(board.team_id, board.id), [board.team_id, board.id]))
  const members = useLoad(useCallback(() => api.members.list(board.team_id), [board.team_id]))
  const action  = useAction(todos.reload)

  const create = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault()
    const form  = event.currentTarget
    const title = String(new FormData(form).get('title'))
    if (await action.run(() => api.todos.create(board.team_id, board.id, title))) {
      form.reset()
    }
  }

  const update = (todo: Todo, patch: Omit<TodoPatch, 'version'>) =>
    action.run(() => api.todos.update(board.team_id, board.id, todo.id, { ...patch, version: todo.version }))

  return (
    <Page
      actions={
        <>
          <button className="button" onClick={onBack} type="button">Back</button>
          <form className="inline-form" onSubmit={create}>
            <input aria-label="Todo title" className="field__input" name="title" placeholder="Todo title" required />
            <button className="button button--primary" type="submit">New todo</button>
          </form>
        </>
      }
      error={action.error ?? todos.error ?? members.error}
      title={board.name}
    >
      {todos.data?.length === 0 && <EmptyState>No todos yet</EmptyState>}
      {!!todos.data?.length && (
        <table className="table">
          <thead>
            <tr>
              <th>Title</th>
              <th>Status</th>
              <th>Assignee</th>
              <th />
            </tr>
          </thead>
          <tbody>
            {todos.data.map((todo) => (
              <tr key={todo.id}>
                <td>{todo.title}</td>
                <td>
                  <select
                    aria-label={`Status of ${todo.title}`}
                    className="field__input"
                    onChange={(event) => update(todo, { status: event.target.value as TodoStatus })}
                    value={todo.status}
                  >
                    {STATUSES.map((status) => (
                      <option key={status} value={status}>{status}</option>
                    ))}
                  </select>
                </td>
                <td>
                  <select
                    aria-label={`Assignee of ${todo.title}`}
                    className="field__input"
                    onChange={(event) => update(todo, { assignee_id: event.target.value || null })}
                    value={todo.assignee_id ?? ''}
                  >
                    <option value="">unassigned</option>
                    {members.data?.map((member) => (
                      <option key={member.user_id} value={member.user_id}>{member.email}</option>
                    ))}
                  </select>
                </td>
                <td className="table__actions">
                  <button className="button button--danger" onClick={() => action.run(() => api.todos.delete(board.team_id, board.id, todo.id))} type="button">
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
