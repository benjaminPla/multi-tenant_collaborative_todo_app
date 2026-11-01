import { api, ApiError, isConflict } from '../../api.ts'
import { Page } from '../../components/Page/Page.tsx'
import { useAction, useLoad } from '../../hooks.ts'
import type { Board as BoardType, Todo, TodoPatch, TodoStatus } from '../../types.ts'
import './Board.css'
import { useCallback, useState, type DragEvent, type FormEvent } from 'react'

type Props = {
  board:  BoardType
  onBack: () => void
}

const COLUMNS: { label: string; status: TodoStatus }[] = [
  { label: 'To do',       status: 'todo' },
  { label: 'In progress', status: 'in-progress' },
  { label: 'Done',        status: 'done' },
]

export function Board({ board, onBack }: Props) {
  const todos           = useLoad(useCallback(() => api.todos.list(board.team_id, board.id), [board.team_id, board.id]))
  const members         = useLoad(useCallback(() => api.members.list(board.team_id), [board.team_id]))
  const action          = useAction(todos.reload)
  const [over, setOver] = useState<TodoStatus | null>(null)

  const create = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault()
    const form  = event.currentTarget
    const title = String(new FormData(form).get('title'))
    if (await action.run(() => api.todos.create(board.team_id, board.id, title))) {
      form.reset()
    }
  }

  // on 409, reapply once if the other edit touched different fields (three-way merge), else keep theirs
  const update = (base: Todo, patch: Omit<TodoPatch, 'version'>) =>
    action.run(async () => {
      try {
        await api.todos.update(board.team_id, board.id, base.id, { ...patch, version: base.version })
      } catch (err) {
        if (!isConflict(err)) {
          throw err
        }
        const theirs = await api.todos.get(board.team_id, board.id, base.id)
        const fields = Object.keys(patch) as (keyof typeof patch)[]
        if (fields.some((field) => theirs[field] !== base[field])) {
          throw new ApiError(409, `"${theirs.title}" was changed by someone else, showing the latest`)
        }
        await api.todos.update(board.team_id, board.id, base.id, { ...patch, version: theirs.version })
      }
    })

  const drop = (event: DragEvent<HTMLElement>, status: TodoStatus) => {
    event.preventDefault()
    setOver(null)
    const todo = todos.data?.find((t) => t.id === event.dataTransfer.getData('text/plain'))
    if (!todo || todo.status === status) {
      return
    }
    todos.setData(todos.data!.map((t) => (t.id === todo.id ? { ...t, status } : t)))
    update(todo, { status })
  }

  const leave = (event: DragEvent<HTMLElement>) => {
    if (!event.currentTarget.contains(event.relatedTarget as Node | null)) {
      setOver(null)
    }
  }

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
      <div className="kanban">
        {COLUMNS.map((column) => {
          const cards = todos.data?.filter((todo) => todo.status === column.status) ?? []
          return (
            <section
              aria-label={column.label}
              className={over === column.status ? 'kanban__column kanban__column--over' : 'kanban__column'}
              key={column.status}
              onDragLeave={leave}
              onDragOver={(event) => {
                event.preventDefault()
                event.dataTransfer.dropEffect = 'move'
                setOver(column.status)
              }}
              onDrop={(event) => drop(event, column.status)}
            >
              <h2 className="kanban__title">
                {column.label}
                <span className="kanban__count">{cards.length}</span>
              </h2>
              <ul className="kanban__cards">
                {cards.map((todo) => (
                  <li
                    className="card"
                    draggable
                    key={todo.id}
                    onDragStart={(event) => {
                      event.dataTransfer.effectAllowed = 'move'
                      event.dataTransfer.setData('text/plain', todo.id)
                    }}
                  >
                    <p className="card__title">{todo.title}</p>
                    <div className="card__footer">
                      <select
                        aria-label={`Assignee of ${todo.title}`}
                        className="field__input card__assignee"
                        onChange={(event) => update(todo, { assignee_id: event.target.value || null })}
                        value={todo.assignee_id ?? ''}
                      >
                        <option value="">unassigned</option>
                        {members.data?.map((member) => (
                          <option key={member.user_id} value={member.user_id}>{member.email}</option>
                        ))}
                      </select>
                      <button
                        aria-label={`Delete ${todo.title}`}
                        className="button button--danger"
                        onClick={() => action.run(() => api.todos.delete(board.team_id, board.id, todo.id))}
                        type="button"
                      >
                        Delete
                      </button>
                    </div>
                  </li>
                ))}
              </ul>
            </section>
          )
        })}
      </div>
    </Page>
  )
}
