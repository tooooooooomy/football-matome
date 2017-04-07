export const RECEIVE_FEEDS = 'RECEIVE_FEEDS'

import urijs from 'urijs'

export const receiveFeeds = (json) => ({
  type: RECEIVE_FEEDS,
  feeds: json.data,
})

export const fetchFeeds = () => (dispatch, getState) => {
  const path = 'http://192.168.33.10/football-matome/api/get'

  return fetch(path, { credentials: 'same-origin' })
        .then(response => response.json())
        .then(json => dispatch(receiveFeeds(json)))
}
