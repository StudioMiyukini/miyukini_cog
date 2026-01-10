import { useState } from 'react'
import type { UserType } from '@/components/hooks/useAuth'

const roles: UserType[] = ['visitor', 'gestion', 'admin', 'superadmin']

export function useRoleSimulation() {
  const [currentRole, setCurrentRole] = useState<UserType>('superadmin')

  const availableRoles = roles.filter((role) => roles.indexOf(role) <= roles.indexOf(currentRole))

  return {
    currentRole,
    availableRoles,
    switchRole: (role: UserType) => {
      if (roles.includes(role)) {
        setCurrentRole(role)
      }
    }
  }
}
