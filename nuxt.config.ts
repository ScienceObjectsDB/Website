// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  devtools: { enabled: true },
  css: [
    '~/assets/styles/main.css',
  ],
  postcss: {
    plugins: {
      tailwindcss: {},
      autoprefixer: {},
    },
  },
  app: {
    head: {
      title: 'Aruna',
    }
  },
  image: {
    dir: 'assets/imgs',
  },


  plugins: [
    "~/plugins/preline.client.ts",
  ],
  modules: [
    '@nuxt/image',
    'nuxt-openid-connect'
  ],

  runtimeConfig: {
    openidConnect: {
      op: {
        issuer: 'http://localhost:1998/realms/test',
        clientId: 'test',
        clientSecret: 'QgBl9I2CD3eVhL7LFvkHrYUK7oKL3LE2',
        callbackUrl: '',
      },
      config: {
        cookieFlags: {
          access_token: {
            httpOnly: true,
            secure: false,
          }
        }
      }
    },
    serverHostUrl: "http://localhost:8080"
  },

  openidConnect: {
    addPlugin: true,
    op: {
      issuer: 'http://localhost:1998/realms/test',
      clientId: 'test',
      clientSecret: 'QgBl9I2CD3eVhL7LFvkHrYUK7oKL3LE2',
      callbackUrl: '',
    },
    config: {
      debug: false,
      response_type: 'code',
      secret: 'oidc._sessionid',
      isCookieUserInfo: false,
      cookie: { loginName: '' },
      cookiePrefix: 'oidc._',
      cookieEncrypt: true,
      cookieEncryptKey: 'bfnuxt9c2470cb477d907b1e0917oidc',
      cookieEncryptIV: 'ab83667c72eec9e4',
      cookieEncryptALGO: 'aes-256-cbc',
      cookieMaxAge: 60 * 10, // 10 Minutes
      cookieFlags: {
        access_token: {
          httpOnly: true,
          secure: false,
        }
      }
    }
  }

})
