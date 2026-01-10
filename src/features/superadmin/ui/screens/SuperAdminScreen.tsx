import { AppShellScreen } from '@/components/layouts/AppShellScreen'

export function SuperAdminScreen() {
  return (
    <AppShellScreen>
      <div className="content-stack">
        <h2>Super Admin</h2>
        <p>Dashboards, audit et impersonation.</p>
      </div>
    </AppShellScreen>
  )
}
