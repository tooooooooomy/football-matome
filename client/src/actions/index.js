export const RECEIVE_FEEDS = 'RECEIVE_FEEDS'
export const REQUEST_FEEDS = 'REQUEST_FEEDS'

import urijs from 'urijs'

export const requestFeeds = () => ({
  type: REQUEST_FEEDS,
})

export const receiveFeeds = (json) => ({
  type: RECEIVE_FEEDS,
  feeds: json.data,
})

export const fetchFeeds = () => (dispatch, getState) => {
  const path = 'api/get'
  return fetch(path, { credentials: 'same-origin' })
        .then(response => response.json())
        .then(json => dispatch(receiveFeeds(json)))
}
