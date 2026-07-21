import { defineConfig } from 'vitepress'

export default defineConfig({
  head: [
    // Tutto first-party. 'unsafe-inline' serve perche' VitePress emette
    // uno script inline per il tema e stili inline.
    [
      'meta',
      {
        'http-equiv': 'Content-Security-Policy',
        content:
          "default-src 'self'; script-src 'self' 'unsafe-inline'; " +
          "style-src 'self' 'unsafe-inline'; img-src 'self' data:; " +
          "font-src 'self'; connect-src 'self'; base-uri 'self'; form-action 'self'",
      },
    ],
  ],
  title: "Eternal-Stream",
  description: "Enterprise Data-to-Video Storage Solution",
  base: "/b2v/", // Assumes deploying to https://username.github.io/b2v/
  themeConfig: {
    footer: {
      message:
        '<a href="https://fabriziosalmi.github.io/privacy">Privacy &amp; legal</a>',
    },
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
      { icon: 'github', link: 'https://github.com/fabriziosalmi/b2v' }
    ]
  }
})
