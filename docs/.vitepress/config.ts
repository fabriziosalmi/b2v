import { defineConfig } from 'vitepress'

export default defineConfig({
  title: "Eternal-Stream",
  description: "Enterprise Data-to-Video Storage Solution",
  base: "/b2v/", // Assumes deploying to https://username.github.io/b2v/
  themeConfig: {
    nav: [
      { text: 'Home', link: '/' },
      { text: 'Guide', link: '/guide/getting-started' }
    ],

    sidebar: [
      {
        text: 'Guide',
        items: [
          { text: 'Getting Started', link: '/guide/getting-started' },
          { text: 'Recommended Platforms', link: '/guide/recommended-platforms' },
          { text: 'Architecture', link: '/guide/architecture' }
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
      { icon: 'github', link: 'https://github.com/your-username/b2v' }
    ]
  }
})
