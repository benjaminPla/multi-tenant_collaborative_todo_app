import { errorMessage } from './api.ts'
import { useEffect, useState } from 'react'

export type Loaded<T> = {
  data?:  T
  error:  string | null
  reload: () => void
}

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

  return { data, error, reload: () => setVersion((v) => v + 1) }
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
