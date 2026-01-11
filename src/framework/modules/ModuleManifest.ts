export type ModuleNavigationEntry = {
  label: string
  href: string
  icon?: string
}

export type ModuleManifest = {
  id: string
  name: string
  version: string
  capabilitiesUsed: readonly string[]
  permissions?: {
    requiredRoles?: readonly string[]
  }
  navigation?: {
    group?: string
    entries: readonly ModuleNavigationEntry[]
  }
  events?: {
    published?: readonly string[]
    subscribed?: readonly string[]
  }
}

