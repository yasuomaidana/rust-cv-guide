import {themes as prismThemes} from 'prism-react-renderer';
import type {Config} from '@docusaurus/types';
import type * as Preset from '@docusaurus/preset-classic';

const config: Config = {
  title: 'Rust CV Guide',
  tagline: 'Pure-Rust answers for OpenCV users',
  favicon: 'img/logo.svg',
  url: 'https://yasuomaidana.github.io',
  baseUrl: '/rust-cv-guide/',
  organizationName: 'yasuomaidana',
  projectName: 'rust-cv-guide',
  trailingSlash: false,
  onBrokenLinks: 'throw',
  onBrokenMarkdownLinks: 'throw',
  markdown: {format: 'detect'},
  presets: [
    [
      'classic',
      {
        docs: {sidebarPath: './sidebars.ts', routeBasePath: '/'},
        blog: false,
        theme: {customCss: './src/css/custom.css'},
      } satisfies Preset.Options,
    ],
  ],
  themes: [
    [
      require.resolve('@easyops-cn/docusaurus-search-local'),
      {hashed: true, indexDocs: true, indexBlog: false, indexPages: false, docsRouteBasePath: '/', highlightSearchTermsOnTargetPage: true},
    ],
  ],
  themeConfig: {
    navbar: {
      title: 'Rust CV Guide',
      logo: {alt: 'Rust CV Guide', src: 'img/logo.svg'},
      items: [
        {type: 'docSidebar', sidebarId: 'docsSidebar', position: 'left', label: 'Guide'},
        {href: 'https://github.com/yasuomaidana/rust-cv-guide', label: 'GitHub', position: 'right'},
      ],
    },
    footer: {
      style: 'dark',
      copyright: 'Not affiliated with OpenCV. Guide content is original; links point to docs.opencv.org.',
    },
    prism: {
      theme: prismThemes.github,
      darkTheme: prismThemes.dracula,
      additionalLanguages: ['bash', 'toml', 'yaml', 'rust'],
    },
  } satisfies Preset.ThemeConfig,
};

export default config;
