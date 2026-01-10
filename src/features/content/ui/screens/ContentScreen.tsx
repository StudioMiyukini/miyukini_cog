'use client'

import { useState } from 'react'
import { AppShellScreen } from '@/components/layouts/AppShellScreen'
import { ContentStack } from '@/components/layouts/ContentStack'
import { BottomNav } from '@/components/navigation/BottomNav'
import { Card, CardHeader, CardBody } from '@/components/atoms/card'
import { Badge } from '@/components/atoms/badge'
import { Avatar } from '@/components/atoms/avatar'

/**
 * ContentScreen - Écran de contenu mock
 * 
 * @layer screens (portable vers Android/iOS)
 * @contract ScreenContract { render, props, navigation }
 * @dependencies FlyonUI, Card, Badge, Avatar
 */

interface ContentItem {
  id: string
  title: string
  description: string
  author: {
    name: string
    avatar?: string
  }
  category: string
  date: string
  readTime: string
  image?: string
}

export interface ContentScreenProps {
  onContentClick?: (id: string) => void
  onCategoryClick?: (category: string) => void
}

// Mock content data
const mockContent: ContentItem[] = [
  {
    id: '1',
    title: 'Introduction au développement mobile cross-platform',
    description: 'Découvrez les meilleures pratiques pour créer des applications mobiles qui fonctionnent sur iOS et Android.',
    author: { name: 'Marie Laurent' },
    category: 'Développement',
    date: '2026-01-08',
    readTime: '5 min',
  },
  {
    id: '2',
    title: 'Design System : les fondamentaux',
    description: 'Apprenez à construire un système de design cohérent et évolutif pour vos projets.',
    author: { name: 'Pierre Martin' },
    category: 'Design',
    date: '2026-01-07',
    readTime: '8 min',
  },
  {
    id: '3',
    title: 'Tailwind CSS avancé : astuces et techniques',
    description: 'Maîtrisez les fonctionnalités avancées de Tailwind CSS pour des interfaces modernes.',
    author: { name: 'Sophie Dubois' },
    category: 'CSS',
    date: '2026-01-06',
    readTime: '6 min',
  },
  {
    id: '4',
    title: 'Architecture React : patterns et bonnes pratiques',
    description: 'Structurez vos applications React de manière maintenable et scalable.',
    author: { name: 'Lucas Bernard' },
    category: 'React',
    date: '2026-01-05',
    readTime: '10 min',
  },
]

const categories = ['Tous', 'Développement', 'Design', 'CSS', 'React', 'TypeScript']

export function ContentScreen({ onContentClick, onCategoryClick }: ContentScreenProps) {
  const [selectedCategory, setSelectedCategory] = useState('Tous')
  const [searchQuery, setSearchQuery] = useState('')

  const filteredContent = mockContent.filter(item => {
    const matchesCategory = selectedCategory === 'Tous' || item.category === selectedCategory
    const matchesSearch = item.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
      item.description.toLowerCase().includes(searchQuery.toLowerCase())
    return matchesCategory && matchesSearch
  })

  const handleCategorySelect = (category: string) => {
    setSelectedCategory(category)
    onCategoryClick?.(category)
  }

  return (
    <AppShellScreen>
      <ContentStack>
        {/* Header */}
        <div className="space-y-4">
          <div>
            <h1 className="text-2xl font-bold text-base-content">Contenu</h1>
            <p className="text-base-content/60">Explorez nos articles et tutoriels</p>
          </div>

          {/* Search */}
          <div className="input input-bordered flex items-center gap-2">
            <span className="icon-[tabler--search] size-5 text-base-content/50" />
            <input
              type="search"
              placeholder="Rechercher..."
              className="grow bg-transparent border-none focus:outline-none"
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
            />
            {searchQuery && (
              <button 
                className="btn btn-ghost btn-circle btn-xs"
                onClick={() => setSearchQuery('')}
              >
                <span className="icon-[tabler--x] size-4" />
              </button>
            )}
          </div>
        </div>

        {/* Categories */}
        <div className="flex gap-2 overflow-x-auto pb-2 -mx-4 px-4 scrollbar-hide">
          {categories.map((category) => (
            <button
              key={category}
              className={`btn btn-sm whitespace-nowrap ${
                selectedCategory === category 
                  ? 'btn-primary' 
                  : 'btn-ghost bg-base-200'
              }`}
              onClick={() => handleCategorySelect(category)}
            >
              {category}
            </button>
          ))}
        </div>

        {/* Featured Content */}
        {filteredContent.length > 0 && selectedCategory === 'Tous' && (
          <Card className="bg-gradient-to-br from-primary/20 to-secondary/20 border-none">
            <CardBody>
              <Badge variant="primary" className="mb-3">À la une</Badge>
              <h2 className="text-xl font-bold text-base-content mb-2">
                {filteredContent[0].title}
              </h2>
              <p className="text-base-content/70 mb-4">
                {filteredContent[0].description}
              </p>
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-2">
                  <Avatar size="sm" />
                  <span className="text-sm text-base-content/80">{filteredContent[0].author.name}</span>
                </div>
                <button 
                  className="btn btn-primary btn-sm"
                  onClick={() => onContentClick?.(filteredContent[0].id)}
                >
                  Lire
                  <span className="icon-[tabler--arrow-right] size-4" />
                </button>
              </div>
            </CardBody>
          </Card>
        )}

        {/* Content List */}
        <div className="space-y-4">
          <h3 className="text-lg font-semibold text-base-content">
            {selectedCategory === 'Tous' ? 'Derniers articles' : selectedCategory}
            <span className="text-base-content/50 font-normal ml-2">({filteredContent.length})</span>
          </h3>

          {filteredContent.length === 0 ? (
            <Card>
              <CardBody className="text-center py-12">
                <span className="icon-[tabler--file-search] size-16 text-base-content/30 mx-auto mb-4" />
                <p className="text-base-content/60">Aucun contenu trouvé</p>
                <button 
                  className="btn btn-ghost btn-sm mt-4"
                  onClick={() => {
                    setSearchQuery('')
                    setSelectedCategory('Tous')
                  }}
                >
                  Réinitialiser les filtres
                </button>
              </CardBody>
            </Card>
          ) : (
            filteredContent.slice(selectedCategory === 'Tous' ? 1 : 0).map((item) => (
              <Card 
                key={item.id} 
                className="cursor-pointer hover:shadow-lg transition-shadow"
                onClick={() => onContentClick?.(item.id)}
              >
                <CardBody>
                  <div className="flex gap-4">
                    {/* Thumbnail placeholder */}
                    <div className="shrink-0 size-20 bg-base-200 rounded-lg flex items-center justify-center">
                      <span className="icon-[tabler--article] size-8 text-base-content/30" />
                    </div>
                    
                    <div className="flex-1 min-w-0">
                      <div className="flex items-start justify-between gap-2">
                        <Badge variant="neutral" size="xs" className="shrink-0">
                          {item.category}
                        </Badge>
                        <span className="text-xs text-base-content/50">{item.readTime}</span>
                      </div>
                      
                      <h4 className="font-semibold text-base-content mt-1 line-clamp-2">
                        {item.title}
                      </h4>
                      
                      <p className="text-sm text-base-content/60 mt-1 line-clamp-2">
                        {item.description}
                      </p>
                      
                      <div className="flex items-center gap-2 mt-2">
                        <Avatar size="xs" src={item.author.avatar} />
                        <span className="text-xs text-base-content/70">{item.author.name}</span>
                        <span className="text-xs text-base-content/50">•</span>
                        <span className="text-xs text-base-content/50">
                          {new Date(item.date).toLocaleDateString('fr-FR', { day: 'numeric', month: 'short' })}
                        </span>
                      </div>
                    </div>
                  </div>
                </CardBody>
              </Card>
            ))
          )}
        </div>
      </ContentStack>
      <BottomNav />
    </AppShellScreen>
  )
}
