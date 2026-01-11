import type { BlockRegistryPort, BlockTypeDefinition } from '../../ports/BlockRegistryPort'

export class InMemoryBlockRegistry implements BlockRegistryPort {
  private map = new Map<string, BlockTypeDefinition>()

  register(def: BlockTypeDefinition): void {
    this.map.set(def.blockType, def)
  }

  list(): BlockTypeDefinition[] {
    return Array.from(this.map.values()).sort((a, b) => a.blockType.localeCompare(b.blockType))
  }

  get(blockType: string): BlockTypeDefinition | null {
    return this.map.get(blockType) ?? null
  }
}

export const maPageBlockRegistry = new InMemoryBlockRegistry()

