import assert from 'power-assert'
import fetchMock from 'fetch-mock'
import thunk from 'redux-thunk'
import configMockStore from 'redux-mock-store'

const middleware = [thunk]
const mockStore = configMockStore(middleware)

import {requestFeeds, receiveFeeds, fetchFeeds } from './'

describe('requstFeeds', () => {
    it ('should return REQUEST_FEEDS action', () => {
        assert.deepStrictEqual(
          requestFeeds(),
          {
            type: 'REQUEST_FEEDS',
          }
        )
    })
})

describe('receiveFeeds', () => {
    it ('should return RECEIVE_FEEDS action', () => {
        const feeds = [{
            id: 1,
            title: 'hoge',
            link: 'http://hoge.com',
        }]

        assert.deepStrictEqual(
            receiveFeeds({data : feeds}),
            {
                type: 'RECEIVE_FEEDS',
                feeds: feeds,
            }
        )
    })
})

describe('fetchFeeds', () => {
  it('should set feeds by response', (done) => {
    const store = mockStore({
      feeds: [],
    })

    const response = {
      data: [
        {
          id: 1,
          title: 'hoge',
          link: 'http://google.com',
        }
      ]
    }

    fetchMock.getOnce('http://192.168.33.10/football-matome/api/get', response)

    store.dispatch(fetchFeeds())
      .then(() => {
        const dispatchedActions = store.getActions()

        assert.strictEqual(dispatchedActions.length, 1)
        assert.deepStrictEqual(dispatchedActions[0], receiveFeeds(response))

        done()
      })
  })
})
