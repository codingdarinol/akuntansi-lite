import { createRouter, createWebHashHistory } from 'vue-router'

import routes from './routes'

const router = createRouter({
  history: createWebHashHistory(),
  routes,
})

router.afterEach((to) => {
  const title = to.meta?.title
  if (typeof title === 'string' && title.length > 0) {
    document.title = `LokalBuku • ${title}`
  } else {
    document.title = 'LokalBuku'
  }
})

export default router
