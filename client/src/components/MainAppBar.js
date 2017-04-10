import React from 'react'
import AppBar from 'material-ui/AppBar'

const MainAppBar = () => (
  <AppBar
    showMenuIconButton={false}
    title="Footballまとめ"
    style={{ position: 'fixed', top: 0, margin: 0 }}
  />
)

export default MainAppBar
