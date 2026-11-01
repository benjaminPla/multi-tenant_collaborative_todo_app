import { EmptyState } from '../components/EmptyState.tsx'
import { Page } from '../components/Page.tsx'

export function Members() {
  return (
    <Page action="Add member" title="Members">
      <EmptyState>No members yet</EmptyState>
    </Page>
  )
}
