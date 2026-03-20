import { ref } from 'vue'

export type NoticeType = 'success' | 'warning' | 'error'

export interface NoticeItem {
  id: number
  type: NoticeType
  text: string
}

// 一个list表，存储当前的通知消息
const notices = ref<NoticeItem[]>([])
let noticeSeed = 0

// 移除所有通知消息
function removeNotice(id: number) {
  notices.value = notices.value.filter((item) => item.id !== id)
}

// 添加通知消息，3秒后自动移除
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

