import assert from 'power-assert'

import { feeds } from './'

describe('feeds', () => {
  it('should return initial state', () => {
    assert.deepStrictEqual(feeds(undefined, {}), {})
  })

  it('should handle REQUEST_FEEDS action', () => {
    const action = {
      type: 'REQUEST_FEEDS',
    }

    assert.deepStrictEqual(feeds(undefined, action), {
      isFetching: true,
    })
  })

  it('should handle RECEIVE_FEEDS action', () => {
    const feedsArray = [{
      id: 1,
      title: 'hoge',
      link: 'http://google.com',
    }]
    const action = {
      type: 'RECEIVE_FEEDS',
      feeds: feedsArray,
    }

    assert.deepStrictEqual(feeds(undefined, action), {
      isFetching: false,
      items: feedsArray,
    })
  })
})
