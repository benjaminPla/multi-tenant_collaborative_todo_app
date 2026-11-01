import { EmptyState } from '../components/EmptyState.tsx'
import { Page } from '../components/Page.tsx'

export function Boards() {
  return (
    <Page action="New board" title="Boards">
      <EmptyState>No boards yet</EmptyState>
    </Page>
  )
}
