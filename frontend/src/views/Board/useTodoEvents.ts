import type { Todo } from '../../types.ts'
import { useEffect, useRef } from 'react'

export type TodoEvent = {
  actor: { email: string; id: string }
  todo:  Todo
}

export type TodoEventType = 'created' | 'deleted' | 'updated'

type Handlers = {
  onClosed: () => void
  onEvent:  (type: TodoEventType, event: TodoEvent) => void
  onResync: () => void
}

const TYPES: TodoEventType[] = ['created', 'deleted', 'updated']

export function useTodoEvents(teamId: string, boardId: string, handlers: Handlers) {
  const latest = useRef(handlers)

  useEffect(() => {
    latest.current = handlers
  })

  useEffect(() => {
    const source = new EventSource(`/api/teams/${teamId}/boards/${boardId}/todos/events`)
    let opened   = false

    // a reopen means events may have been missed while disconnected
    source.addEventListener('open', () => {
      if (opened) {
        latest.current.onResync()
      }
      opened = true
    })
    source.addEventListener('error', () => {
      if (source.readyState === EventSource.CLOSED) {
        latest.current.onClosed()
      }
    })
    source.addEventListener('resync', () => latest.current.onResync())
    for (const type of TYPES) {
      source.addEventListener(`todo.${type}`, (event) => {
        latest.current.onEvent(type, JSON.parse((event as MessageEvent<string>).data))
      })
    }

    return () => source.close()
  }, [teamId, boardId])
}
