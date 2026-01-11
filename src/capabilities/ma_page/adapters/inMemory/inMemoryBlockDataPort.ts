import type { BlockDataPort, BlockDataResolver } from '../../ports/BlockDataPort'

export class InMemoryBlockDataPort implements BlockDataPort {
  private resolvers = new Map<string, BlockDataResolver>()

  registerResolver(blockType: string, resolver: BlockDataResolver): void {
    this.resolvers.set(blockType, resolver)
  }

  async resolve(blockType: string, dataRef: unknown, context: { viewerUserId?: string | null }) {
    const resolver = this.resolvers.get(blockType)
    if (!resolver) return null
    return await resolver({ blockType, dataRef, context })
  }
}

export const maPageBlockDataPort = new InMemoryBlockDataPort()

