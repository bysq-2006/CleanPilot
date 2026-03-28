import { invoke } from '@tauri-apps/api/core'
import { ref } from 'vue'

export type AgentMessageRole = 'system' | 'user' | 'assistant' | 'tool'

export interface AgentToolFunction {
  name: string
  arguments: string
}

export interface AgentToolCall {
  id: string
  type: string
  function: AgentToolFunction
}

export interface AgentMessage {
  role: AgentMessageRole
  content?: string | null
  tool_calls?: AgentToolCall[] | null
  tool_call_id?: string | null
}

type HistorySyncState = 'idle' | 'active'

const idleIntervalMs = 600
const activeIntervalMs = 20
const activeTimeoutMs = 1000

export class AgentHistoryStore {
  history = ref<AgentMessage[]>([])
  index = ref(0)
  state = ref<HistorySyncState>('idle')
  syncError = ref<string | null>(null)
  isPolling = ref(false)

  private timer: number | null = null
  private isSyncing = false
  private activeUntil = 0

  constructor() {
    this.start()
  }

  /** 根据当前状态决定下一次同步间隔。 */
  private getInterval() {
    return this.state.value === 'active' ? activeIntervalMs : idleIntervalMs
  }

  /** 按“1000ms 内是否出现新增项”规则回写轮询状态。 */
  private updateStateByTime(now = Date.now()) {
    this.state.value = now < this.activeUntil ? 'active' : 'idle'
  }

  /** 采用串行 setTimeout，确保一次同步完成后再调度下一次。 */
  private scheduleNextTick() {
    if (!this.isPolling.value) return

    this.updateStateByTime()

    this.timer = window.setTimeout(() => {
      void this.tick()
    }, this.getInterval())
  }

  /** 从尾部对齐 history：0 条不处理；其余按“覆盖最后一项 + 追加新增项”处理。 */
  private mergeHistory(incoming: AgentMessage[]) {
    if (incoming.length === 0) return

    if (this.history.value.length === 0) {
      this.history.value = incoming
      this.index.value = this.history.value.length === 0 ? 0 : this.history.value.length - 1
      return
    }

    if (incoming.length === 1) {
      const [nextMessage] = incoming
      const lastMessage = this.history.value[this.history.value.length - 1]

      const shouldReplaceLastMessage =
        lastMessage?.role === 'assistant'
        && nextMessage.role === 'assistant'

      this.history.value = shouldReplaceLastMessage
        ? [...this.history.value.slice(0, -1), nextMessage]
        : [...this.history.value, nextMessage]

      this.index.value = this.history.value.length - 1
      return
    }

    this.history.value = [...this.history.value.slice(0, -1), ...incoming]
    this.index.value = this.history.value.length - 1
  }

  /** 执行一次同步：只要本次有消息就刷新 active 窗口；无消息则按超时回到 idle。 */
  async tick() {
    if (this.isSyncing) return

    this.isSyncing = true

    try {
      const incoming = await invoke<AgentMessage[]>('get_history', {
        startIndex: this.index.value,
      })

      this.mergeHistory(incoming)

      if (incoming.length > 0) {
        this.activeUntil = Date.now() + activeTimeoutMs
      }

      this.syncError.value = null
    }
    catch (error) {
      const message = error instanceof Error ? error.message : String(error)
      this.syncError.value = message
    }
    finally {
      this.isSyncing = false
      this.scheduleNextTick()
    }
  }

  /** 启动内部轮询器；实例创建后会自动调用一次。 */
  start() {
    if (this.isPolling.value) return

    this.isPolling.value = true
    void this.tick()
  }

  /** 停止内部轮询器并清理定时器。 */
  stop() {
    this.isPolling.value = false

    if (this.timer !== null) {
      window.clearTimeout(this.timer)
      this.timer = null
    }
  }
}
