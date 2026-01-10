export type UserType = 'visitor' | 'gestion' | 'admin' | 'superadmin'

export function useAuth() {
  const user = {
    id: 'superadmin',
    email: 'miyukini@gmail.com',
    user_type: 'superadmin' as UserType,
    role: 'super admin'
  }

  return {
    user,
    token: 'dev-token',
    isLoading: false,
    login: async (_email: string, _password: string) => user,
    logout: async () => null
  }
}
