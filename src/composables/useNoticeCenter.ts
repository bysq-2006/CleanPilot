import { ref } from 'vue'

export type NoticeType = 'success' | 'warning' | 'error'

export interface NoticeItem {
  id: number
  type: NoticeType
  text: string
}

const notices = ref<NoticeItem[]>([])
let noticeSeed = 0

function removeNotice(id: number) {
  notices.value = notices.value.filter((item) => item.id !== id)
}

export function pushNotice(type: NoticeType, text: string) {
  const id = ++noticeSeed
  notices.value.unshift({ id, type, text })

  window.setTimeout(() => {
    removeNotice(id)
  }, 3000)
}

export function useNoticeCenter() {
  return {
    notices,
    pushNotice,
    removeNotice,
  }
}

