export const maPageCapabilityManifest = {
  id: 'ma_page',
  name: 'Ma Page',
  version: '0.1.0',

  exports: {
    ports: ['ProfileTypePort', 'ProfilePagePort', 'BlockRegistryPort', 'BlockDataPort'],
    hooks: ['useMaPage', 'useMaPageEvents'],
  },

  dataContracts: {
    tables: ['profile_types', 'profile_type_rules', 'profile_pages', 'profile_blocks', 'profile_tags'],
  },

  policies: {
    profile_types: ['write:admin_super_admin'],
    profile_type_rules: ['write:admin_super_admin'],
    profile_pages: ['select:public_or_owner', 'write:owner_or_admin'],
    profile_blocks: ['select:page_visibility', 'write:owner_or_admin'],
  },

  eventsEmitted: [
    'profile.type.created',
    'profile.type.updated',
    'profile.type.assigned',
    'profile.page.updated',
    'profile.block.updated',
    'profile.page.published',
    'profile.page.unpublished',
  ],

  eventsConsumed: [
    'agenda.slot.confirmed',
    'billing.invoice.paid',
    'rpg.profile.updated',
    'documents.file.uploaded',
  ],
} as const

