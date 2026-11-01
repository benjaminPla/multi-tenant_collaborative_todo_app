import type { Notice } from '../../hooks.ts'
import './Toasts.css'

export function Toasts({ notices }: { notices: Notice[] }) {
  return (
    <div aria-live="polite" className="toasts" role="status">
      {notices.map((notice) => (
        <p className="toast" key={notice.id}>
          {notice.text}
        </p>
      ))}
    </div>
  )
}
