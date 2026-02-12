/* ~~/extension/utils/message.ts */

// imports
import { LIBRI_DICTATION_MESSAGE, LIBRI_DICTATION_ON_MESSAGE } from '../utils/constant'

export interface Message {
  id: typeof LIBRI_DICTATION_MESSAGE | typeof LIBRI_DICTATION_ON_MESSAGE
  payload: Array<Event | OnEvent>
}

export type Event = 'DictationPanelOpenStatus' | 'OpenDictationPanel' | 'PageUnload'

interface DictationPanelOpenStatus {
  DictationPanelOpenStatus: boolean
}

export type OnEvent = DictationPanelOpenStatus
