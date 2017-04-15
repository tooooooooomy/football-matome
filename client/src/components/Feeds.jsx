import React, { PropTypes } from 'react'
import { Table, TableBody, TableRow, TableRowColumn } from 'material-ui/Table'

const Feeds = ({ feeds, onClick }) => (
  <Table onCellClick={onClick}>
    <TableBody displayRowCheckbox={false}>
      {feeds.map((feed, i) =>
        <TableRow key={i}>
          <TableRowColumn>
            {feed.title}
          </TableRowColumn>
        </TableRow>,
    )}
    </TableBody>
  </Table>
)

Feeds.propTypes = {
  feeds: PropTypes.array.isRequired,
  onClick: PropTypes.func.isRequired,
}

export default Feeds
