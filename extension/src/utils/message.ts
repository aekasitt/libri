/* ~~/extension/src/utils/message.ts */

// imports
import type { Event, OnEvent, Message } from '../types/message'
import { LIBRI_DICTATION_MESSAGE, LIBRI_DICTATION_ON_MESSAGE } from './constant'

/**
 * TODO
 */
export function createMessage(payload: Event | Array<Event> | undefined): Message {
  if (typeof payload === 'undefined') {
    payload = []
  } else if (!Array.isArray(payload)) {
    payload = [payload]
  }
  return {
    id: LIBRI_DICTATION_MESSAGE,
    payload,
  }
}

/**
 * TODO
 */
export function createOnMessage(payload: OnEvent | Array<OnEvent> | undefined): Message {
  if (typeof payload === 'undefined') {
    payload = []
  } else if (!Array.isArray(payload)) {
    payload = [payload]
  }
  return {
    id: LIBRI_DICTATION_ON_MESSAGE,
    payload,
  }
}
