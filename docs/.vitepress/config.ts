import { defineConfig } from 'vitepress'

export default defineConfig({
  title: "Eternal-Stream",
  description: "Enterprise Data-to-Video Storage Solution",
  base: "/b2v/", // Assumes deploying to https://username.github.io/b2v/
  lastUpdated: true,
  cleanUrls: true,
  themeConfig: {
    nav: [
      { text: 'Home', link: '/' },
      { text: 'Guide', link: '/guide/getting-started' },
      { text: 'API Reference', link: '/guide/api' },
      { text: 'Contributing', link: '/contributing' }
    ],

    sidebar: [
      {
        text: 'Getting Started',
        items: [
          { text: 'Introduction', link: '/' },
          { text: 'Quick Start', link: '/guide/getting-started' },
          { text: 'Recommended Platforms', link: '/guide/recommended-platforms' }
        ]
      },
      {
        text: 'Architecture',
        items: [
          { text: 'Overview', link: '/guide/architecture' },
          { text: 'Design Patterns', link: '/guide/architecture/design-patterns' }
        ]
      },
      {
        text: 'Code Internals',
        items: [
          { text: 'CLI Entry (main.rs)', link: '/guide/code/main' },
          { text: 'Utils & Header', link: '/guide/code/utils' },
          { text: 'Encoder Logic', link: '/guide/code/encoder' },
          { text: 'Decoder Logic', link: '/guide/code/decoder' }
        ]
      }
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/fabriziosalmi/b2v' }
    ],

    editLinkText: "Edit this page on GitHub",
    outline: ['deep']
  },

  vite: {
    optimizeDeps: {
      include: []
    }
  }
})
