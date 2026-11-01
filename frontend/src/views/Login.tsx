import { api, ApiError } from '../api.ts'
import type { User } from '../types.ts'
import { useState, type FormEvent } from 'react'

type Props = {
  onLogin: (user: User) => void
}

export function Login({ onLogin }: Props) {
  const [error, setError]     = useState<string | null>(null)
  const [pending, setPending] = useState(false)

  const submit = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault()
    const form = new FormData(event.currentTarget)
    setError(null)
    setPending(true)
    try {
      await api.login(String(form.get('email')), String(form.get('password')))
      onLogin(await api.me())
    } catch (err) {
      setError(err instanceof ApiError ? err.message : 'something went wrong')
    } finally {
      setPending(false)
    }
  }

  return (
    <main className="login">
      <form className="login__card" onSubmit={submit}>
        <h1 className="login__title">plaza</h1>
        <label className="field">
          <span className="field__label">Email</span>
          <input autoComplete="username" className="field__input" name="email" required type="email" />
        </label>
        <label className="field">
          <span className="field__label">Password</span>
          <input autoComplete="current-password" className="field__input" name="password" required type="password" />
        </label>
        {error && <p className="form__error" role="alert">{error}</p>}
        <button className="button button--primary" disabled={pending} type="submit">
          Sign in
        </button>
      </form>
    </main>
  )
}
