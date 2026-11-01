import { EmptyState } from '../components/EmptyState.tsx'
import { Page } from '../components/Page.tsx'

export function Users() {
  return (
    <Page action="New user" title="Users">
      <EmptyState>No users yet</EmptyState>
    </Page>
  )
}
