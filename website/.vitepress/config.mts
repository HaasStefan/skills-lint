import { defineConfig } from 'vitepress'

export default defineConfig({
  title: 'skills-lint',
  description: 'Token budget linter for agent skill files',
  base: '/skills-lint/',

  head: [
    ['link', { rel: 'icon', href: '/skills-lint/favicon.svg', type: 'image/svg+xml' }],
  ],

  themeConfig: {
    nav: [
      { text: 'Guide', link: '/guide/' },
      { text: 'Reference', link: '/reference/cli' },
      {
        text: 'GitHub',
        link: 'https://github.com/HaasStefan/skills-lint',
      },
    ],

    sidebar: {
      '/guide/': [
        {
          text: 'Guide',
          items: [
            { text: 'Getting Started', link: '/guide/' },
            { text: 'Configuration', link: '/guide/configuration' },
            { text: 'CI Integration', link: '/guide/ci-integration' },
          ],
        },
      ],
      '/reference/': [
        {
          text: 'Reference',
          items: [
            { text: 'CLI', link: '/reference/cli' },
            { text: 'Config Schema', link: '/reference/config-schema' },
            { text: 'Encodings', link: '/reference/encodings' },
          ],
        },
      ],
    },

    search: {
      provider: 'local',
    },

    socialLinks: [
      { icon: 'github', link: 'https://github.com/HaasStefan/skills-lint' },
    ],

    footer: {
      message: 'Released under the MIT License.',
      copyright: 'Copyright © 2025 Stefan Haas',
    },
  },
})
