import { api } from '../api.ts'
import { EmptyState } from '../components/EmptyState.tsx'
import { Page } from '../components/Page.tsx'
import { formatDate } from '../format.ts'
import { useAction, useLoad } from '../hooks.ts'
import { Board } from './Board.tsx'
import { useCallback, useState, type FormEvent } from 'react'

export function Boards({ teamId }: { teamId: string }) {
  const boards              = useLoad(useCallback(() => api.boards.list(teamId), [teamId]))
  const action              = useAction(boards.reload)
  const [openId, setOpenId] = useState<string | null>(null)
  const open                = boards.data?.find((board) => board.id === openId)

  if (open) {
    return <Board board={open} onBack={() => setOpenId(null)} />
  }

  const create = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault()
    const form = event.currentTarget
    const name = String(new FormData(form).get('name'))
    if (await action.run(() => api.boards.create(teamId, name))) {
      form.reset()
    }
  }

  return (
    <Page
      actions={
        <form className="inline-form" onSubmit={create}>
          <input aria-label="Board name" className="field__input" name="name" placeholder="Board name" required />
          <button className="button button--primary" type="submit">New board</button>
        </form>
      }
      error={action.error ?? boards.error}
      title="Boards"
    >
      {boards.data?.length === 0 && <EmptyState>No boards yet</EmptyState>}
      {!!boards.data?.length && (
        <table className="table">
          <thead>
            <tr>
              <th>Name</th>
              <th>Created</th>
              <th />
            </tr>
          </thead>
          <tbody>
            {boards.data.map((board) => (
              <tr key={board.id}>
                <td>
                  <button className="button--link" onClick={() => setOpenId(board.id)} type="button">
                    {board.name}
                  </button>
                </td>
                <td>{formatDate(board.created_at)}</td>
                <td className="table__actions">
                  <button className="button button--danger" onClick={() => action.run(() => api.boards.delete(teamId, board.id))} type="button">
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
