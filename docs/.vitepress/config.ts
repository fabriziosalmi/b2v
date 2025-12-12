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
          { text: 'Architecture', link: '/guide/architecture' },
          { text: 'Benchmarks', link: '/guide/benchmarks' }
        ]
      }
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/your-username/b2v' }
    ]
  }
})
