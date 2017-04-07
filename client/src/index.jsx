import React from 'react'
import ReactDom from 'react-dom'
import { createStore, combineReducers } from 'redux'
import { Provider } from 'react-redux'
import { Router, Route, IndexRoute, browserHistory } from 'react-router'
import { syncHistoryWithStore, routerReducer } from 'react-router-redux'
import { App, Home, Foo, Bar } from './components'

import reducers from 'reducers'

const store = createStore(
    combineReducers({
      ...reducers,
      routing: routerReducer,
    }),
)

const history = syncHistoryWithStore(browserHistory, store)

ReactDom.render(
    <Provider store={store}>
        <Router history={history}>
            <Route path="/" component={App}>
            </Route>
        </Router>
    </Provider>,
    document.getElementById('root'),
)
