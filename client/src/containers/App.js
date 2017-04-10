import React, { Component, PropTypes } from 'react'
import { connect } from 'react-redux'
import { fetchFeeds, openLink } from '../actions'
import Feeds from '../components/Feeds'
import MainAppBar from '../components/MainAppBar'
import MuiThemeProvider from 'material-ui/styles/MuiThemeProvider';

class App extends Component {
  static PropTypes = {
    feeds: PropTypes.array.isRequired,
    isFetching: PropTypes.bool.isRequired,
    dispatch: PropTypes.func.isRequired,
  }

  componentDidMount() {
    const { dispatch } = this.props
    dispatch(fetchFeeds())
  }

  handleClick = ( row, column, event ) => {
    const { feeds } = this.props
    openLink(feeds[row].link)
  }

  render() {
    const { feeds, isFetching } = this.props
    const isEmpty = feeds === undefined || feeds.length === 0
    return (
      <MuiThemeProvider>
        <div style={{ paddingTop: 64 }}>
          <MainAppBar />
          {isEmpty ? (isFetching ? <h2>Loading...</h2> : <h2>Empty.</h2>)
          : <div style = {{ opacity: isFetching ? 0.5 : 1 }}>
              <Feeds
                feeds={feeds}
                onClick={this.handleClick}
              />
            </div>
          }
        </div>
      </MuiThemeProvider>
    )
  }
}

const mapStateToProps = state => {
  const {
    isFetching,
    items: feeds
  } = state.feeds || {
    isFetching: true,
    items: [],
  }

  return {
    feeds,
    isFetching,
  }
}

export default connect(mapStateToProps)(App)
