export const RECEIVE_FEEDS = 'RECEIVE_FEEDS'
export const REQUEST_FEEDS = 'REQUEST_FEEDS'
export const SELECT_FEED = 'SELECT_FEED'

export const requestFeeds = () => ({
  type: REQUEST_FEEDS,
})

export const receiveFeeds = json => ({
  type: RECEIVE_FEEDS,
  feeds: json.data,
})

export const fetchFeeds = () => (dispatch) => {
  const path = 'api/get'
  return fetch(path, { credentials: 'same-origin' })
        .then(response => response.json())
        .then(json => dispatch(receiveFeeds(json)))
}

export const openLink = (link, target = "_blank") => {
  window.open(link, target)
}
