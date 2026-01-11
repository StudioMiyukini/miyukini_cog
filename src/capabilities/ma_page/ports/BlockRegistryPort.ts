export type BlockConfigValidationResult =
  | { ok: true }
  | { ok: false; errors: string[] }

export type BlockTypeDefinition = {
  blockType: string
  label: string
  description?: string
  /**
   * Si false, ce bloc ne doit jamais être affiché sur une page publique.
   * (La policy RLS doit aussi garantir le filtrage côté DB si besoin.)
   */
  canBePublic?: boolean
  /**
   * Validation de config (schema zod/json-schema côté module).
   * Ici on reste agnostique: le module fournit une fonction.
   */
  validateConfig?: (config: unknown) => BlockConfigValidationResult
}

export type BlockRegistryPort = {
  register(def: BlockTypeDefinition): void
  list(): BlockTypeDefinition[]
  get(blockType: string): BlockTypeDefinition | null
}

