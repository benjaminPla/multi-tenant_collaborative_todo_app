import { errorMessage } from './api.ts'
import { useCallback, useEffect, useRef, useState } from 'react'

export type Loaded<T> = {
  data?:  T
  error:  string | null
  reload: () => void
  update: (change: (data: T) => T) => void
}

export type Notice = {
  id:   number
  text: string
}

const NOTICE_MS = 4000

export function useLoad<T>(load: () => Promise<T>): Loaded<T> {
  const [data, setData]       = useState<T>()
  const [error, setError]     = useState<string | null>(null)
  const [version, setVersion] = useState(0)

  useEffect(() => {
    let active = true
    load()
      .then((result) => {
        if (active) {
          setData(result)
          setError(null)
        }
      })
      .catch((err) => active && setError(errorMessage(err)))
    return () => {
      active = false
    }
  }, [load, version])

  return {
    data,
    error,
    reload: () => setVersion((v) => v + 1),
    update: (change) => setData((current) => (current === undefined ? current : change(current))),
  }
}

export function useAction(onSettled: () => void) {
  const [error, setError] = useState<string | null>(null)

  const run = async (action: () => Promise<unknown>) => {
    setError(null)
    try {
      await action()
      return true
    } catch (err) {
      setError(errorMessage(err))
      return false
    } finally {
      onSettled()
    }
  }

  return { error, run }
}

export function useNotices() {
  const [notices, setNotices] = useState<Notice[]>([])
  const nextId                = useRef(0)

  const notify = useCallback((text: string) => {
    const id = nextId.current++
    setNotices((current) => [...current, { id, text }])
    setTimeout(() => setNotices((current) => current.filter((notice) => notice.id !== id)), NOTICE_MS)
  }, [])

  return { notices, notify }
}
